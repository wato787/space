# Chapter 15: Smart Pointers

## TL;DR

- `Box<T>` でヒープ確保する
- `Rc<T>` で共有所有、`RefCell<T>` で内部可変性

## 学習メモ

- `RefCell` は実行時に借用規則を検査する

## 実行

- `rustc main.rs && ./main`
