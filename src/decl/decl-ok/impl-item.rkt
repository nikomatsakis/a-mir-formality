#lang racket
(require redex/reduction-semantics
         "../../logic/grammar.rkt"
         "../../logic/env.rkt"
         "../../logic/substitution.rkt"
         "../../ty/user-ty.rkt"
         "../grammar.rkt"
         "../where-clauses.rkt"
         "../feature-gate.rkt"
         "../builtin-predicate.rkt"
         )
(provide impl-item-ok-goal
         )

(define-metafunction formality-decl
  ;; Given the definition of an `ImplItem`, along wiht
  ;;
  ;; * the program test (`CrateDecls`)
  ;;
  ;; produces a Goal that proves the impl item to be well-formed.
  ;;
  ;; The Goal expects to be proven in the context of the impl, meaning that the impl where-clauses
  ;; are assumed to hold, and that the inputs to the impl are well-formed.
  impl-item-ok-goal : CrateDecls TraitId UserParameters ImplItem -> Goal

  [(impl-item-ok-goal CrateDecls TraitId (UserParameter_trait ...) FnDecl)
   Goal_implies

   ; unpack things
   (where/error (fn FnId (KindedVarId ...) (Ty_arg ...) -> Ty_ret where [WhereClause ...] _) FnDecl)

   ; find the declaration of this associated type
   (where/error (trait TraitId KindedVarIds_trait where WhereClauses_trait TraitItems_trait) (trait-with-id CrateDecls TraitId))
   (where/error (_ ... (fn FnId KindedVarIds_trait-fn (Ty_trait-fn-arg ...) -> Ty_trait-fn-ret where WhereClauses_trait-fn _) _ ...) TraitItems_trait)
   (where/error ((_ VarId_trait) ...) KindedVarIds_trait)

   ; validate that parameter-kinds match and are same in number in trait and on impl
   ;
   ; FIXME WF checking in mir layer should be ensuring this
   ;
   ; FIXME -- Rust's early/late-bound logic is different here, we may want to make this less conservative
   (where/error (((ParameterKind VarId_trait-fn) ..._generics) ((ParameterKind VarId) ..._generics))
                (KindedVarIds_trait-fn (KindedVarId ...))
                )
   (where/error ((_ ..._args) (_ ..._args))
                ((Ty_arg ...) (Ty_trait-fn-arg ...))
                )

   ; create a substitution from the variables in the impl to the trait
   (where/error Substitution_trait->impl-user ((VarId_trait UserParameter_trait) ... (VarId_trait-fn VarId) ...))
   (where/error Substitution_trait->impl ((VarId_trait (user-parameter UserParameter_trait)) ... (VarId_trait-fn VarId) ...))

   ; validate that, for all values of the impl-fn's type parameters...
   ;
   ; (a) argument types from trait-fn are subtypes of impl-fn's argument types
   ; (b) return type from impl-fn is a subtype of trait-fn's return type
   ; (c) where-clauses from impl-fn are provable using where clauses from trait
   (where/error [Ty_trait-fn-arg-s ...] [(apply-substitution Substitution_trait->impl Ty_trait-fn-arg) ...])
   (where/error Ty_trait-fn-ret-s (apply-substitution Substitution_trait->impl Ty_trait-fn-ret))
   (where/error Goal_implies
                (∀ [KindedVarId ...]
                   (implies
                    (where-clauses->hypotheses CrateDecls (apply-substitution Substitution_trait->impl-user WhereClauses_trait-fn))
                    (&& [(Ty_trait-fn-arg-s <= Ty_arg) ... ; (a)
                         (Ty_ret <= Ty_trait-fn-ret-s) ; (b)
                         (where-clause->goal CrateDecls WhereClause) ... ; (c)
                         ]
                        ))))
   ]

  [; For an associated type value
   ;
   ; impl LendingIterator for Bar {
   ;     type Item<'a> = u32;
   ; }
   ;
   ; given
   ;
   ; trait LendingIterator {
   ;     type Item<'a> : Sized
   ;     where Self: 'a;
   ; }
   ;
   ; To be valid, the following conditions must hold:
   ;
   ; (a) `u32` is well-formed, given the where-clauses on the associated value from the impl (empty set, in this example);
   ; (b) `u32` must implement the bounds from the trait (`Sized`)
   ; (c) the where-clauses in the trait (`where Self: 'a`) must imply those from the impl (empty set, in this example)
   ;
   ; Note that the where-clauses from the enclosing impl (along with the generics on that impl)
   ; are declared by our caller, `crate-item-ok-goal`.

   (impl-item-ok-goal CrateDecls TraitId (UserParameter_trait ...) AssociatedTyValue)
   (∀ (KindedVarId ...)
      (implies [(well-formed KindedVarId) ...]
               (&& [Goal_ty-wf-and-meets-bounds ; (a) and (b)
                    Goal_implies ; (c) where-clauses in trait imply those on the impl
                    ])
               )
      )

   ; unpack things
   (where/error (type AssociatedTyId (KindedVarId ...) = Ty where WhereClauses) AssociatedTyValue)

   ; find the declaration of this associated type
   (where/error (trait TraitId KindedVarIds_trait where WhereClauses_trait TraitItems_trait) (trait-with-id CrateDecls TraitId))
   (where/error (_ ... (type AssociatedTyId KindedVarIds_trait-ty BoundsClause_trait-ty where WhereClauses_trait-ty) _ ...) TraitItems_trait)
   (where/error ((_ VarId_trait) ...) KindedVarIds_trait)

   ; validate that parameter-kinds match and are same in number in trait and on impl
   (where/error (((ParameterKind VarId_trait-ty) ..._same) ((ParameterKind VarId) ..._same))
                (KindedVarIds_trait-ty (KindedVarId ...))
                )

   ; create a substitution from the variables in the impl to the trait
   (where/error Substitution_trait->impl ((VarId_trait UserParameter_trait) ... (VarId_trait-ty VarId) ...))

   ; goals to check that `Ty` meets the bounds declared in trait
   (where/error WhereClauses_bty (instantiate-bounds-clause BoundsClause_trait-ty (internal Ty)))
   (where/error (Goal_bty ...) (where-clauses->goals CrateDecls WhereClauses_bty))

   ; goals (a) and (b) -- the type `Ty` meets its bounds and is well-formed,
   ; assuming the where-clauses on the assoc type impl item hold.
   ;
   ; IMPORTANT: We use `well-formed-goal-for-ty` to create the goal, rather than
   ; writing `(well-formed (type Ty))`. This "breaks the loop" to prevent us from
   ; leveraging this very impl to prove WF. Imagine this program:
   ;
   ; ```
   ; impl Iterator for SomeType { type Item = SomeOtherType<String>; }
   ; struct SomeOtherType<T: Copy> { t: T }
   ; ```
   ;
   ; This other type `SomeOtherType<String>` is not well-formed, so the impl should be invalid.
   ; But if we just tried to prove `(well-formed (type SomeOtherType<String>))`, the solver would be
   ; within its rights to combine the facts that ...
   ;
   ; * `<SomeType as Iterator>::Item` (as an alias) is well-formed, since `SomeType: Iterator`
   ; * `<SomeType as Iterator>::Item` normalizes to `SomeOtherType<String>` (from the impl)
   ; * therefore `SomeOtherType<String>` is WF
   ;
   ; Instead we use `well-formed-goal-for-ty`, which means we have to prove the
   ; where-clause from `SomeOtherType` definition, so we try to prove `String: Copy` (which fails).
   ;
   ; One exception is if we have something like `impl<T> Foo for Blah<T> { type Item = T; }`.
   ; In that case, the `well-formed-goal-for-ty` will just be `(well-formed (type T))`, which is fine
   ; since that is only provable via a hypothesis (that the one who relies on our impl must prove).
   (where/error Goal_ty-wf-and-meets-bounds
                (implies (where-clauses->hypotheses CrateDecls WhereClauses)
                         (&& (Goal_bty ... ; (a) Ty meets bounds declared in trait
                              (well-formed-goal-for-ty CrateDecls Ty) ; (b) type in impl is wf
                              ))))

   ; goal (c) -- the where-clauses from the trait imply the where-clauses from the impl
   (where/error Goal_implies
                (implies
                 (where-clauses->hypotheses CrateDecls (apply-substitution Substitution_trait->impl WhereClauses_trait-ty))
                 (&& (where-clauses->goals CrateDecls WhereClauses))
                 ))
   ]
  )
