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

(define context%
  (class object%
    (define slots (make-hash))
    (define matches '())
    (super-new)
    (define/public (reset)
      (set! matches '()))
    (define/public (matched key)
      (set! matches (append key matches)))
    (define/public (get-matches)
      matches)
    (define/public (matched?)
      (not (empty? matches)))
    (define/public (get-slots)
      slots)))

(define state 'state)
(define message 'message)

(define-syntax (when x)
  (syntax-case x ()
    [(when [(category key value) ...] body ...)
     (with-syntax ([ctx (datum->syntax (syntax when) 'ctx)])
       (syntax
        (lambda (ctx)
          (let* ([slots (send ctx get-slots)]
                 [matches (filter (lambda (kv)
                                    (and (hash-has-key? slots (first kv)) (eq? (hash-ref slots (first kv)) (second kv))))
                                  (list '((category . key) value) ... ))])
            (if (empty? matches) null
                (begin
                  (send ctx matched (map first matches))
                  (body ctx) ...))
            ctx))))]))

(define-syntax (exec x)
  (syntax-case x ()
    [(exec cmd [result action] ...)
     (with-syntax ([ctx (datum->syntax (syntax exec) 'ctx)])
       (syntax
        (lambda (ctx)
          (match (system cmd)
            [result (action ctx)] ...))))]))

(define-syntax (update x)
  (syntax-case x ()
    [(update [(category key value) ...])
     (with-syntax ([ctx (datum->syntax (syntax update) 'ctx)])
       (syntax
        (lambda (ctx)
          ; TODO: reject certain categories like messages?
          (hash-set! (send ctx get-slots) (cons category key) value) ...)))]))

(define-syntax (do x)
  (syntax-case x ()
    [(do f ...)
     (with-syntax ([ctx (datum->syntax (syntax do) 'ctx)])
       (syntax
        (lambda (ctx)
          f ...)))]))

(define-syntax (acknowledge x)
  (syntax-case x ()
    [(acknowledge)
     (with-syntax ([ctx (datum->syntax (syntax do) 'ctx)])
       (syntax
        (lambda (ctx)
          (let ([slots (send ctx get-slots)])
            (map (lambda (k) (hash-remove! slots k))
                 (filter (lambda (k) (eq? (car k) message)) (send ctx get-matches)))))))]))

(define-syntax (react x)
  (syntax-case x (init)
    [(reactor (init [(category key value) ...]) body ...)
     (with-syntax ([ctx (datum->syntax (syntax do) 'ctx)])
       (syntax
        (lambda (arg)
          (let ([ctx (if (is-a? arg context%)
                         (let ([ctx+ arg])
                           (send ctx+ reset)
                           ctx+)
                         (let ([ctx+ (new context%)])
                           (hash-set! (send ctx+ get-slots) (cons category key) value) ...
                           ctx+))])
            (map (lambda (f) (f ctx)) (list body ...))
            ctx))))]))

(define (run r)
  (define (loop ctx)
;    (if (not (send ctx matched?))
;        (begin
;          (println "waiting...")
;          (sleep 1))
;        null)
    (sleep 1)
    (loop (r ctx)))
  (loop (r null)))

(provide (all-defined-out))