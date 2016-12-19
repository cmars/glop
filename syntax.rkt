#lang racket

;   Copyright 2016 Casey Marshall
;
;   Licensed under the Apache License, Version 2.0 (the "License");
;   you may not use this file except in compliance with the License.
;   You may obtain a copy of the License at
;
;       http://www.apache.org/licenses/LICENSE-2.0
;
;   Unless required by applicable law or agreed to in writing, software
;   distributed under the License is distributed on an "AS IS" BASIS,
;   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
;   See the License for the specific language governing permissions and
;   limitations under the License.

(define-syntax (init x)
  (syntax-case x ()
    [(init [(category key value) ...])
     (with-syntax ([ctx (datum->syntax (syntax init) 'ctx)])
       (syntax
        (lambda (ctx)
          (hash-set! ctx '(category . key) value) ...)))]))

(define-syntax (when x)
  (syntax-case x ()
    [(when [(category key value) ...] body ...)
     (with-syntax ([ctx (datum->syntax (syntax when) 'ctx)])
       (syntax
        (lambda (ctx)
          (match ctx
            [(hash-table ('(category . key) value) ...)
             (body ctx) ...
             ; TODO: mark context as having matched
             ; TODO: push message key onto "matched message" stack
             ]))))]))

(define-syntax (exec x)
  (syntax-case x ()
    [(exec cmd [result action] ...)
     (with-syntax ([ctx (datum->syntax (syntax exec) 'ctx)])
       (syntax
        (lambda (ctx)
          (match (system cmd)
            [result (action ctx)] ...))))]))

(define-syntax (update x)
  ; TODO: consolidate with init?
  (syntax-case x ()
    [(update [(category key value) ...])
     (with-syntax ([ctx (datum->syntax (syntax update) 'ctx)])
       (syntax
        (lambda (ctx)
          ; TODO: reject certain categories like messages?
          (hash-set! ctx '(category . key) value) ...)))]))

(define-syntax (do x)
  (syntax-case x ()
    [(do f ...)
     (with-syntax ([ctx (datum->syntax (syntax do) 'ctx)])
       (syntax
        (lambda (ctx)
          f ...)))]))

(define-syntax react
  (syntax-rules ()
    [(react body ...)
     ((lambda ()
        (let ([ctx (make-hash)])
          (body ctx) ...)))]))

(define state 'state)

(provide (all-defined-out))