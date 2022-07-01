#lang racket
(require redex/reduction-semantics
         "../grammar.rkt"
         "../where-clauses.rkt"
         "../feature-gate.rkt"
         "../../logic/env.rkt"
         "../../ty/user-ty.rkt"
         )
(provide impl-item-decl-rules
         )

(define-metafunction formality-decl
  ;; Given the name of a trait and some item within that trait, Return a tuple of:
  ;;
  ;; * The clauses that hold in all crates due to this item
  ;; * The invariants that hold
  impl-item-decl-rules : CrateDecls CrateId (impl KindedVarIds TraitId UserParameters for UserTy where WhereClauses) ImplItem -> (Clauses Invariants)

  [;; For a method declared in a trait: currently no logical rules are created.
   (impl-item-decl-rules CrateDecls CrateId _ FnDecl)
   (() ())
   ]

  [;; For an associated type declared in a trait, defines
   ;;
   ;; * clause for when the alias-ty is well-formed
   ;; *
   (impl-item-decl-rules CrateDecls
                         CrateId
                         (impl [KindedVarId_impl ...] TraitId (UserParameter_trait ...) for UserTy where WhereClauses_impl)
                         AssociatedTyValue)
   ([Clause_norm] [])

   ; unpack things
   (where/error (type AssociatedTyId [KindedVarId_val ...] = Ty where WhereClauses_val) AssociatedTyValue)
   (where/error [(ParameterKind_val VarId_val) ...] [KindedVarId_val ...])

   ; find the declaration of this associated type
   (where/error (trait TraitId KindedVarIds_trait where WhereClauses_trait TraitItems_trait) (trait-with-id CrateDecls TraitId))
   (where/error [_ ... (type AssociatedTyId KindedVarIds_decl BoundsClause_decl where WhereClauses_decl) _ ...] TraitItems_trait)
   (where/error [(_ VarId_trait) ...] KindedVarIds_trait)

   ; create the alias-ty being defined
   (where/error (Parameter_trait ...) ((user-ty UserTy) (user-parameter UserParameter_trait) ...))
   (where/error AliasTy (alias-ty (TraitId AssociatedTyId) [Parameter_trait ... VarId_val ...]))

   (where/error Clause_norm
                (∀ [KindedVarId_impl ... KindedVarId_val ...]
                   (implies
                    (where-clauses->hypotheses
                     CrateDecls
                     (flatten (WhereClauses_impl WhereClauses_val)))
                    (normalizes-to AliasTy Ty))))
   ]
  )
