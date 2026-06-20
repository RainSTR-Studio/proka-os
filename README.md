# Proka OS - A simple, secure and standalone operating system

[![Rust Nightly](https://img.shields.io/badge/rust-nightly-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License: GPLv3](https://img.shields.io/badge/License-GPLv3-yellow.svg?style=flat-square)](https://opensource.org/license/gpl-3.0)
[![GitHub Stars](https://img.shields.io/github/stars/RainSTR-Studio/proka-os?style=flat-square)](https://github.com/RainSTR-Studio/proka-os/stargazers)
[![GitHub Issues](https://img.shields.io/github/issues/RainSTR-Studio/proka-os?style=flat-square)](https://github.com/RainSTR-Studio/proka-os/issues)
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/RainSTR-Studio/proka-os?style=flat-square)](https://github.com/RainSTR-Studio/proka-os/pulls)
[![Documentation](https://img.shields.io/badge/docs-prokadoc-brightgreen?style=flat-square)](https://prokadoc.pages.dev/)

**Copyright (C) 2026 RainSTR Studio. Licensed under GNU GPLv3.**

---

## Introduction
This is the main repository of ProkaOS, which contains the kernel, bootloader and so on, using submodules to manage.

This project has bootstrap to help you build up an image of this operating system, so that you are able to run this operating system through this image.

## Build
To build this project, you need to install these components:

 - Rust (nightly);
 - cargo-anaxa (build script will do);
 - NASM (for assembly build)
 - GCC (for C code generation)

After installing them, you can run build script through this command:

```bash
# Build only
python3 x.py

# Do menu configuration
python3 x.py menuconfig
```

## Contributing
Thank you to all contributors!

 - **zhangxuan2011** <zx20110412@outlook.com>

### How to contribute
We welcome your contributions: Bug reports, Pull Requests (features, fixes, optimizations), documentation improvements, and feedback.

Also don't forget to add your name to [**Contributors List**](#contributors)! :)

# License
This project is under license [**GPL-v3**](LICENSE), and you should follow this license during contributing.

See [LICENSE](LICENSE) for more details.
