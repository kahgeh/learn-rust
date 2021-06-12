# Notes

The way to think about RefCell is a level of indirection in which it's borrow on it's reference are immutable, however underneath it's possible to request a mutable reference. Even though this bypasses the check during compile time, RefCell still ensures the borrow checker rules of many immutables or one immutable reference is adhered to, albeit during run time instead of compile time.

This allows developer to bypass side effects of the borrow checker which exclude valid requirement such as interior mutability
