;;; -*- lexical-binding: t -*-
(require 'json-rpc)
(defun json-rpc--request (connection version endpoint method params)
  (let* ((id (cl-incf (json-rpc-id-counter connection)))
         (request `(:method ,method :params ,params :id ,id))
         (auth (json-rpc-auth connection))
         (process (json-rpc-process (json-rpc-ensure connection)))
         (encoded (if version
                      (json-encode (nconc (list :jsonrpc version) request))
                    (json-encode request))))
    (with-current-buffer (process-buffer (json-rpc-process connection))
      (erase-buffer))
    (with-temp-buffer
      (insert (format "POST %s HTTP/1.1\r\n" (url-encode-url endpoint)))
      (when auth (insert "Authorization: Basic " auth "\r\n"))
      (insert "Content-Type: application/json\r\n")
      (insert (format "Content-Length: %d\r\n\r\n" (string-bytes encoded))
              encoded)
      (process-send-region process (point-min) (point-max)))
    (json-rpc-wait connection)))

(defvar fall-mode-hook nil)

(defvar fall-peer)
(defconst fall-server "/home/matklad/projects/fall/target/debug/epc")

(add-to-list 'auto-mode-alist '("\\.fall\\'" . fall-mode))


(defconst fall-faces
  '(("keyword" . font-lock-keyword-face)
    ("token" . font-lock-constant-face)
    ("rule" . font-lock-type-face)
    ("string" . font-lock-string-face)))


(defun fall-highlight-span (span)
  (let* ((start (elt span 0))
	 (end (elt span 1))
	 (color (elt span 2))
	 (face (cdr (assoc color fall-faces))))
    (add-text-properties
     (+ 1 start) (+ 1 end)
     `(face ,face))))

(defun fall-rehighlight (spans)
  (remove-text-properties 1  (- (buffer-size) 1) '(face nil))
  (seq-doseq (span spans)
    (fall-highlight-span span)))

(defun do-after-text-change (start end old-len)
  (let ((spans (plist-get (json-rpc fall-peer "colors" (buffer-string)) :spans)))
    (fall-rehighlight spans)))

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
