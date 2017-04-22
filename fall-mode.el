;;; -*- lexical-binding: t -*-
(require 'json-rpc)

(defvar fall-mode-hook nil)


(add-to-list 'auto-mode-alist '("\\.fall\\'" . fall-mode))

(defun fall--rpc-colors ()
  (plist-get (json-rpc fall-peer "colors" (buffer-string)) :spans))

(defun fall--rpc-tree ()
  (plist-get (json-rpc fall-peer "tree" (buffer-string)) :tree))

(defun parse-tree ()
  "Display parse tree of the current buffer"
  (interactive)
  (with-output-to-temp-buffer "tree"
    (princ (fall--rpc-tree))))


(defconst fall-faces
  '(("keyword" . font-lock-keyword-face)
    ("token" . font-lock-constant-face)
    ("rule" . font-lock-type-face)
    ("string" . font-lock-string-face)
    ("builtin" . font-lock-builtin-face)))


(defun fall-highlight-span (span)
  (let* ((start (elt span 0))
	 (end (elt span 1))
	 (color (elt span 2)))
    (message "%s %s" color (equal color "error"))
    (if (equal color "error")
	(progn
	  (message "fooo")
	  (add-face-text-property
	   (+ 1 start) (+ 1 end)
	   '(:underline (:color "red" :style wave)))
      	  )
      (add-text-properties
	 (+ 1 start) (+ 1 end)
     	 `(face ,(cdr (assoc color fall-faces)))))))

(defun fall-rehighlight (spans)
  (remove-text-properties 1 (- (buffer-size) 1) '(face nil))
  (seq-doseq (span spans)
    (fall-highlight-span span)))

(defun do-after-text-change (start end old-len)
  (fall-rehighlight (fall--rpc-colors)))

(defun after-text-change (start end old-length)
  (do-after-text-change start end old-length))

(defun fall-mode ()
  "Major mode for editing Fall parser genrator files"
  (interactive)
  (kill-all-local-variables)
  (setq major-mode 'fall-mode)
  (setq mode-name "Fall")
  (add-hook 'after-change-functions 'after-text-change nil 't)
  (setf fall-peer (json-rpc-connect "127.0.0.1" 9292 nil nil))
  (run-hooks 'fall-mode-hook)
  (after-text-change 0 (buffer-size) 0)
  (message "Loaded Fall mode"))



(provide 'fall-mode)
