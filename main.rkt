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

(require "syntax.rkt")

(react
 (init [(state "installed" #f)])
 (when [(state "installed" #f)]
   (do (display "about to install\n"))
   (exec "./doit.bash"
         [#t (do (display "yay\n"))]
         [#f (do (display "boo\n"))]))
   (do (display "done\n")))