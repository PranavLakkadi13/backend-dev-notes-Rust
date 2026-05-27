# SMTP Protocol - Rust Implementation

## Overview
This project is a simple Rust application demonstrating how to send emails programmatically using the `lettre` crate and `dotenvy` for environment variables. 

**Key takeaway from setup:** When using `dotenvy`, values with spaces in the `.env` file (like app passwords) must be wrapped in quotes (e.g., `APP_PASS="xxxx xxxx xxxx xxxx"`). Missing quotes will cause a parsing error, which might be silently ignored if you use `.ok()` during initialization.

---

## Domain Knowledge Q&A: Understanding SMTP & Protocols

**Q: Is SMTP to email what HTTP is to the web? Does it just act as a standard protocol for all providers?**  
**A:** Yes! SMTP (Simple Mail Transfer Protocol) is the universal standard for *sending* and *relaying* outgoing emails. Just like HTTP dictates the structure for web communication (headers, body, methods), SMTP provides the rules (text commands like `EHLO`, `MAIL FROM`, `DATA`) for email clients and servers to understand each other.

**Q: Does our Rust program just send the message to the `smtp.gmail.com` nodes, which then relay it to the destination?**  
**A:** Exactly. Your Rust program acts as a local mail client. It connects and authenticates with Google's MTA (Mail Transfer Agent) via `smtp.gmail.com`. Google's servers then take over, look up the recipient's mail server, and use SMTP to relay the message to the final destination.

**Q: Is the mail encrypted while relaying? How do providers like ProtonMail differ?**  
**A:** Yes, emails are encrypted *in transit* using a mechanism called **STARTTLS**. The connection between your machine and Gmail, and between Gmail and the destination server, is heavily encrypted. However, it is not *End-to-End Encrypted (E2EE)* by default—meaning Gmail's servers process the email in plaintext. Secure providers like ProtonMail solve this by encrypting the actual message *before* it gets sent (PGP encryption), so their servers only ever handle an unreadable ciphertext blob.

**Q: So basically, protocols like HTTP and SMTP just define the structure (headers, auth parameters, body) for how computers should format text to communicate over a network?**  
**A:** 100% correctly assumed. Underneath it all, protocols are nothing more than a shared language or agreed-upon format for sending raw text and bytes over a TCP connection. Whether it's REST over HTTP or emailing over SMTP, it's just standardizing the format so the receiving computer knows how to parse it!