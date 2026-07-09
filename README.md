# [!IMPORTANT]
# imccc
# Just a Compiler

---

> [!NOTE]
> Core Architecture Overview
> This core system automatically intercepts file extensions to seamlessly route files into their optimized pipelines, ensuring zero conflict between binary execution and dynamic interpretation.

---

Technical Routing Matrix

| Execution Model | Supported Language / Extensions |
| :--- | :--- |
| **AOT Compilation** <br> *(Ahead-Of-Time)* | `c` / `cpp` / `cc` / `cxx` / `go` / `rs` / `java` / `cs` / `swift` / `kt` / `kts` |
| **Dynamic Interpretation** <br> *(JIT / VM Runtime)* | `py` / `pyw` / `js` / `mjs` / `ts` / `lua` / `rb` / `php` |

---
 Installation & Deployment

> [!IMPORTANT]
> Make sure your cargo environment is up to date before launching the installation script.

Run the automated script to build and link the binary directly into your path environment:
```bash
./install.sh
