# Unsupported Features

- .com scarf flutter
- .com start rotation

Not supporting these because they use the web browser's trig implementations, which is different across different web engines (SpiderMonkey vs V8), let alone standard libraries of different languages.

- LRA/OpenLR bone rest lengths

These versions calculate bone rest lengths after start offset is applied to the overall entity. While this seems trivial to support, it would have a ripple effect on how the code is structured (eg rest lengths would need to be cached per entity in some separate field) which I cba to implement.

- Flash 6.3/6.7 gravity bug

The flash build for 6.7 and 6.3 use 6.2 grid physics, but their gravity value is slightly off (0.17500000000000002 instead of 0.175). This must be manually set by users, since it is not currently possible to detect which flash build a track came from.
