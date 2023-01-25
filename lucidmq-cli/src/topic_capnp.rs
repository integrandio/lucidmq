// @generated by the capnpc-rust plugin to the Cap'n Proto schema compiler.
// DO NOT EDIT.
// source: topic.capnp


pub mod topic_request {
  pub use self::Which::{Describe,Create,Delete};

  #[derive(Copy, Clone)]
  pub struct Owned(());
  impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

  #[derive(Clone, Copy)]
  pub struct Reader<'a> { reader: ::capnp::private::layout::StructReader<'a> }

  impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>  {
    fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
      Reader { reader,  }
    }
  }

  impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
    fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Reader<'a,>> {
      ::core::result::Result::Ok(::capnp::traits::FromStructReader::new(reader.get_struct(default)?))
    }
  }

  impl <'a,> ::capnp::traits::IntoInternalStructReader<'a> for Reader<'a,>  {
    fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a> {
      self.reader
    }
  }

  impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
    fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
      self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
    }
  }

  impl <'a,> Reader<'a,>  {
    pub fn reborrow(&self) -> Reader<'_,> {
      Reader { .. *self }
    }

    pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
      self.reader.total_size()
    }
    #[inline]
    pub fn get_topic_name(self) -> ::capnp::Result<::capnp::text::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_topic_name(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn which(self) -> ::core::result::Result<WhichReader, ::capnp::NotInSchema> {
      match self.reader.get_data_field::<u16>(0) {
        0 => {
          ::core::result::Result::Ok(Describe(
            ()
          ))
        }
        1 => {
          ::core::result::Result::Ok(Create(
            ()
          ))
        }
        2 => {
          ::core::result::Result::Ok(Delete(
            ()
          ))
        }
        x => ::core::result::Result::Err(::capnp::NotInSchema(x))
      }
    }
  }

  pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
  impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
    #[inline]
    fn struct_size() -> ::capnp::private::layout::StructSize { _private::STRUCT_SIZE }
  }
  impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>  {
    fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
      Builder { builder,  }
    }
  }

  impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
    fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
      self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
    }
  }

  impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
    fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Builder<'a,> {
      ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
    }
    fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Builder<'a,>> {
      ::core::result::Result::Ok(::capnp::traits::FromStructBuilder::new(builder.get_struct(_private::STRUCT_SIZE, default)?))
    }
  }

  impl <'a,> ::capnp::traits::SetPointerBuilder for Reader<'a,>  {
    fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>, value: Reader<'a,>, canonicalize: bool) -> ::capnp::Result<()> { pointer.set_struct(&value.reader, canonicalize) }
  }

  impl <'a,> Builder<'a,>  {
    pub fn into_reader(self) -> Reader<'a,> {
      ::capnp::traits::FromStructReader::new(self.builder.into_reader())
    }
    pub fn reborrow(&mut self) -> Builder<'_,> {
      Builder { .. *self }
    }
    pub fn reborrow_as_reader(&self) -> Reader<'_,> {
      ::capnp::traits::FromStructReader::new(self.builder.into_reader())
    }

    pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
      self.builder.into_reader().total_size()
    }
    #[inline]
    pub fn get_topic_name(self) -> ::capnp::Result<::capnp::text::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_topic_name(&mut self, value: ::capnp::text::Reader<'_>)  {
      self.builder.get_pointer_field(0).set_text(value);
    }
    #[inline]
    pub fn init_topic_name(self, size: u32) -> ::capnp::text::Builder<'a> {
      self.builder.get_pointer_field(0).init_text(size)
    }
    #[inline]
    pub fn has_topic_name(&self) -> bool {
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn set_describe(&mut self, _value: ())  {
      self.builder.set_data_field::<u16>(0, 0);
    }
    #[inline]
    pub fn set_create(&mut self, _value: ())  {
      self.builder.set_data_field::<u16>(0, 1);
    }
    #[inline]
    pub fn set_delete(&mut self, _value: ())  {
      self.builder.set_data_field::<u16>(0, 2);
    }
    #[inline]
    pub fn which(self) -> ::core::result::Result<WhichBuilder, ::capnp::NotInSchema> {
      match self.builder.get_data_field::<u16>(0) {
        0 => {
          ::core::result::Result::Ok(Describe(
            ()
          ))
        }
        1 => {
          ::core::result::Result::Ok(Create(
            ()
          ))
        }
        2 => {
          ::core::result::Result::Ok(Delete(
            ()
          ))
        }
        x => ::core::result::Result::Err(::capnp::NotInSchema(x))
      }
    }
  }

  pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
  impl ::capnp::capability::FromTypelessPipeline for Pipeline {
    fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
      Pipeline { _typeless: typeless,  }
    }
  }
  impl Pipeline  {
  }
  mod _private {
    use capnp::private::layout;
    pub const STRUCT_SIZE: layout::StructSize = layout::StructSize { data: 1, pointers: 1 };
    pub const TYPE_ID: u64 = 0x9969_3f47_4fce_0e40;
  }
  pub enum Which {
    Describe(()),
    Create(()),
    Delete(()),
  }
  pub type WhichReader = Which;
  pub type WhichBuilder = Which;
}

pub mod topic_response {
  pub use self::Which::{Describe,Create,Delete};

  #[derive(Copy, Clone)]
  pub struct Owned(());
  impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

  #[derive(Clone, Copy)]
  pub struct Reader<'a> { reader: ::capnp::private::layout::StructReader<'a> }

  impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>  {
    fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
      Reader { reader,  }
    }
  }

  impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
    fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Reader<'a,>> {
      ::core::result::Result::Ok(::capnp::traits::FromStructReader::new(reader.get_struct(default)?))
    }
  }

  impl <'a,> ::capnp::traits::IntoInternalStructReader<'a> for Reader<'a,>  {
    fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a> {
      self.reader
    }
  }

  impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
    fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
      self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
    }
  }

  impl <'a,> Reader<'a,>  {
    pub fn reborrow(&self) -> Reader<'_,> {
      Reader { .. *self }
    }

    pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
      self.reader.total_size()
    }
    #[inline]
    pub fn get_topic_name(self) -> ::capnp::Result<::capnp::text::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_topic_name(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_success(self) -> bool {
      self.reader.get_bool_field(0)
    }
    #[inline]
    pub fn which(self) -> ::core::result::Result<WhichReader<'a,>, ::capnp::NotInSchema> {
      match self.reader.get_data_field::<u16>(1) {
        0 => {
          ::core::result::Result::Ok(Describe(
            ::capnp::traits::FromStructReader::new(self.reader)
          ))
        }
        1 => {
          ::core::result::Result::Ok(Create(
            ()
          ))
        }
        2 => {
          ::core::result::Result::Ok(Delete(
            ()
          ))
        }
        x => ::core::result::Result::Err(::capnp::NotInSchema(x))
      }
    }
  }

  pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
  impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
    #[inline]
    fn struct_size() -> ::capnp::private::layout::StructSize { _private::STRUCT_SIZE }
  }
  impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>  {
    fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
      Builder { builder,  }
    }
  }

  impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
    fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
      self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
    }
  }

  impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
    fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Builder<'a,> {
      ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
    }
    fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Builder<'a,>> {
      ::core::result::Result::Ok(::capnp::traits::FromStructBuilder::new(builder.get_struct(_private::STRUCT_SIZE, default)?))
    }
  }

  impl <'a,> ::capnp::traits::SetPointerBuilder for Reader<'a,>  {
    fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>, value: Reader<'a,>, canonicalize: bool) -> ::capnp::Result<()> { pointer.set_struct(&value.reader, canonicalize) }
  }

  impl <'a,> Builder<'a,>  {
    pub fn into_reader(self) -> Reader<'a,> {
      ::capnp::traits::FromStructReader::new(self.builder.into_reader())
    }
    pub fn reborrow(&mut self) -> Builder<'_,> {
      Builder { .. *self }
    }
    pub fn reborrow_as_reader(&self) -> Reader<'_,> {
      ::capnp::traits::FromStructReader::new(self.builder.into_reader())
    }

    pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
      self.builder.into_reader().total_size()
    }
    #[inline]
    pub fn get_topic_name(self) -> ::capnp::Result<::capnp::text::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_topic_name(&mut self, value: ::capnp::text::Reader<'_>)  {
      self.builder.get_pointer_field(0).set_text(value);
    }
    #[inline]
    pub fn init_topic_name(self, size: u32) -> ::capnp::text::Builder<'a> {
      self.builder.get_pointer_field(0).init_text(size)
    }
    #[inline]
    pub fn has_topic_name(&self) -> bool {
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_success(self) -> bool {
      self.builder.get_bool_field(0)
    }
    #[inline]
    pub fn set_success(&mut self, value: bool)  {
      self.builder.set_bool_field(0, value);
    }
    #[inline]
    pub fn init_describe(self, ) -> crate::topic_capnp::topic_response::describe::Builder<'a> {
      self.builder.set_data_field::<u16>(1, 0);
      self.builder.set_data_field::<u64>(1, 0u64);
      self.builder.set_data_field::<u64>(2, 0u64);
      self.builder.get_pointer_field(1).clear();
      ::capnp::traits::FromStructBuilder::new(self.builder)
    }
    #[inline]
    pub fn set_create(&mut self, _value: ())  {
      self.builder.set_data_field::<u16>(1, 1);
    }
    #[inline]
    pub fn set_delete(&mut self, _value: ())  {
      self.builder.set_data_field::<u16>(1, 2);
    }
    #[inline]
    pub fn which(self) -> ::core::result::Result<WhichBuilder<'a,>, ::capnp::NotInSchema> {
      match self.builder.get_data_field::<u16>(1) {
        0 => {
          ::core::result::Result::Ok(Describe(
            ::capnp::traits::FromStructBuilder::new(self.builder)
          ))
        }
        1 => {
          ::core::result::Result::Ok(Create(
            ()
          ))
        }
        2 => {
          ::core::result::Result::Ok(Delete(
            ()
          ))
        }
        x => ::core::result::Result::Err(::capnp::NotInSchema(x))
      }
    }
  }

  pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
  impl ::capnp::capability::FromTypelessPipeline for Pipeline {
    fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
      Pipeline { _typeless: typeless,  }
    }
  }
  impl Pipeline  {
  }
  mod _private {
    use capnp::private::layout;
    pub const STRUCT_SIZE: layout::StructSize = layout::StructSize { data: 3, pointers: 2 };
    pub const TYPE_ID: u64 = 0xbd62_f653_7db6_6e92;
  }
  pub enum Which<A0> {
    Describe(A0),
    Create(()),
    Delete(()),
  }
  pub type WhichReader<'a,> = Which<crate::topic_capnp::topic_response::describe::Reader<'a>>;
  pub type WhichBuilder<'a,> = Which<crate::topic_capnp::topic_response::describe::Builder<'a>>;

  pub mod describe {
    #[derive(Copy, Clone)]
    pub struct Owned(());
    impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
    impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
    impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

    #[derive(Clone, Copy)]
    pub struct Reader<'a> { reader: ::capnp::private::layout::StructReader<'a> }

    impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
      #[inline]
      fn type_id() -> u64 { _private::TYPE_ID }
    }
    impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>  {
      fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
        Reader { reader,  }
      }
    }

    impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
      fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Reader<'a,>> {
        ::core::result::Result::Ok(::capnp::traits::FromStructReader::new(reader.get_struct(default)?))
      }
    }

    impl <'a,> ::capnp::traits::IntoInternalStructReader<'a> for Reader<'a,>  {
      fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a> {
        self.reader
      }
    }

    impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
      fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
        self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
      }
    }

    impl <'a,> Reader<'a,>  {
      pub fn reborrow(&self) -> Reader<'_,> {
        Reader { .. *self }
      }

      pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
        self.reader.total_size()
      }
      #[inline]
      pub fn get_max_segment_bytes(self) -> u64 {
        self.reader.get_data_field::<u64>(1)
      }
      #[inline]
      pub fn get_max_retention_bytes(self) -> u64 {
        self.reader.get_data_field::<u64>(2)
      }
      #[inline]
      pub fn get_consumer_groups(self) -> ::capnp::Result<::capnp::text_list::Reader<'a>> {
        ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(1), ::core::option::Option::None)
      }
      #[inline]
      pub fn has_consumer_groups(&self) -> bool {
        !self.reader.get_pointer_field(1).is_null()
      }
    }

    pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
    impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
      #[inline]
      fn struct_size() -> ::capnp::private::layout::StructSize { _private::STRUCT_SIZE }
    }
    impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
      #[inline]
      fn type_id() -> u64 { _private::TYPE_ID }
    }
    impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>  {
      fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
        Builder { builder,  }
      }
    }

    impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
      fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
        self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
      }
    }

    impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
      fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Builder<'a,> {
        ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
      }
      fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Builder<'a,>> {
        ::core::result::Result::Ok(::capnp::traits::FromStructBuilder::new(builder.get_struct(_private::STRUCT_SIZE, default)?))
      }
    }

    impl <'a,> ::capnp::traits::SetPointerBuilder for Reader<'a,>  {
      fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>, value: Reader<'a,>, canonicalize: bool) -> ::capnp::Result<()> { pointer.set_struct(&value.reader, canonicalize) }
    }

    impl <'a,> Builder<'a,>  {
      pub fn into_reader(self) -> Reader<'a,> {
        ::capnp::traits::FromStructReader::new(self.builder.into_reader())
      }
      pub fn reborrow(&mut self) -> Builder<'_,> {
        Builder { .. *self }
      }
      pub fn reborrow_as_reader(&self) -> Reader<'_,> {
        ::capnp::traits::FromStructReader::new(self.builder.into_reader())
      }

      pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
        self.builder.into_reader().total_size()
      }
      #[inline]
      pub fn get_max_segment_bytes(self) -> u64 {
        self.builder.get_data_field::<u64>(1)
      }
      #[inline]
      pub fn set_max_segment_bytes(&mut self, value: u64)  {
        self.builder.set_data_field::<u64>(1, value);
      }
      #[inline]
      pub fn get_max_retention_bytes(self) -> u64 {
        self.builder.get_data_field::<u64>(2)
      }
      #[inline]
      pub fn set_max_retention_bytes(&mut self, value: u64)  {
        self.builder.set_data_field::<u64>(2, value);
      }
      #[inline]
      pub fn get_consumer_groups(self) -> ::capnp::Result<::capnp::text_list::Builder<'a>> {
        ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(1), ::core::option::Option::None)
      }
      #[inline]
      pub fn set_consumer_groups(&mut self, value: ::capnp::text_list::Reader<'a>) -> ::capnp::Result<()> {
        ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(1), value, false)
      }
      #[inline]
      pub fn init_consumer_groups(self, size: u32) -> ::capnp::text_list::Builder<'a> {
        ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(1), size)
      }
      #[inline]
      pub fn has_consumer_groups(&self) -> bool {
        !self.builder.get_pointer_field(1).is_null()
      }
    }

    pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
    impl ::capnp::capability::FromTypelessPipeline for Pipeline {
      fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
        Pipeline { _typeless: typeless,  }
      }
    }
    impl Pipeline  {
    }
    mod _private {
      use capnp::private::layout;
      pub const STRUCT_SIZE: layout::StructSize = layout::StructSize { data: 3, pointers: 2 };
      pub const TYPE_ID: u64 = 0x85ec_2351_de2b_2d66;
    }
  }
}
