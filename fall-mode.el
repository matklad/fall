;;; -*- lexical-binding: t -*-
(require 'json-rpc)

(defvar fall-mode-hook nil)

(defvar fall-peer)
(defconst fall-server "/home/matklad/projects/fall/target/debug/epc")

(add-to-list 'auto-mode-alist '("\\.fall\\'" . fall-mode))

(defun fall-highlith (start end type)
  (message "%S %S %S" start end type)
  ;; (add-face-text-property start end '(:foreground "red"))
  (add-text-properties (+ 1 start) (+ 1 end) '(face font-lock-keyword-face))
  )

(defun do-after-text-change (start end old-len)
  (let ((spans (plist-get (json-rpc fall-peer "colors" (buffer-string)) :spans)))
    (seq-doseq (s spans)
      (apply 'fall-highlith (append s nil)))))

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
  (message "Loaded Fall mode"))



(provide 'fall-mode)
