#lang racket
(require redex/reduction-semantics
         "../../logic/grammar.rkt"
         "../grammar.rkt"
         "../where-clauses.rkt"
         "../feature-gate.rkt"
         "../../logic/env.rkt"
         "wf-where-clause.rkt"
         )
(provide trait-item-ok-goal
         )
(define-metafunction formality-decl
  trait-item-ok-goal : CrateDecls (TraitId KindedVarIds_trait WhereClauses_trait) TraitItem -> Goal

  [(trait-item-ok-goal CrateDecls (TraitId KindedVarIds_trait WhereClauses_trait) FnDecl)
   true-goal
   ]

  [;; Given an associated like:
   ;;
   ;;    type Item<'a>: Sized where Self: 'a;
   ;;
   ;; we have to prove that the where-clause `Self: 'a` is well-formed as is the bound `Sized`.
   (trait-item-ok-goal CrateDecls (TraitId (KindedVarId_trait ...) (WhereClause_trait ...)) AssociatedTyDecl)
   (∀ (KindedVarId_trait ...)
      (implies ((where-clause->hypothesis CrateDecls WhereClause_trait) ...
                (well-formed KindedVarId_trait) ...
                (is-implemented (TraitId (VarId_trait ...)))
                )
               (∀ (KindedVarId ...)
                  (implies ((where-clause->hypothesis CrateDecls WhereClause) ...
                            (well-formed KindedVarId) ...
                            )
                           (&& (Goal_wc-wf ...
                                Goal_bound-wf ...
                                )
                               )

                           ))))

   ; unpack associated type declaration
   (where/error (type AssociatedTyId (KindedVarId ...) BoundsClause where (WhereClause ...)) AssociatedTyDecl)
   (where/error ((_ VarId_trait) ...) (KindedVarId_trait ...))
   (where/error ((_ VarId) ...) (KindedVarId ...))


   ; requirements for where-clauses (e.g., `Self: 'a`) to be wf:
   (where/error (Goal_wc-wf ...) ((well-formed-where-clause-goal CrateDecls WhereClause) ...))

   ; requirements for bound (`Sized`) to be wf:
   (where/error AliasTy (alias-ty (TraitId AssociatedTyId) (VarId_trait ... VarId ...)))
   (where/error (WhereClause_bound ...) (instantiate-bounds-clause BoundsClause (internal AliasTy)))
   (where/error (Goal_bound-wf ...) ((well-formed-where-clause-goal CrateDecls WhereClause_bound) ...))
   ]
  )
