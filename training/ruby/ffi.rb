require 'ffi'

module Rust
  extend FFI::Library
  ffi_lib 'target/release/liblcs.dylib'
  attach_function :lcs_ruby, [:string, :string], :uint
end

s = "abcd"
t = "becd"

p Rust.lcs_ruby(s, t)
