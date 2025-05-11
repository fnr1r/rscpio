#[macro_export]
macro_rules! impl_binread_with_mini {
    ($t:ty) => {
        impl ::minibinrw::binrw::BinRead for $t {
            type Args<'a> = ();
            fn read_options<R: ::std::io::Read + ::std::io::Seek>(
                reader: &mut R,
                endian: ::minibinrw::binrw::Endian,
                _args: Self::Args<'_>,
            ) -> ::minibinrw::binrw::BinResult<Self> {
                let pos = reader.stream_position()?;
                Self::m_read_options(reader, endian)
                    .map_err(|e| ::minibinrw::MiniBinError::into_binrw(e, pos))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_binwrite_with_mini {
    ($t:ty) => {
        impl ::minibinrw::binrw::BinWrite for $t {
            type Args<'a> = ();
            fn write_options<W: ::std::io::Write + ::std::io::Seek>(
                &self,
                writer: &mut W,
                endian: ::minibinrw::binrw::Endian,
                _args: Self::Args<'_>,
            ) -> ::minibinrw::binrw::BinResult<()> {
                let pos = writer.stream_position()?;
                self.m_write_options(writer, endian)
                    .map_err(|e| ::minibinrw::MiniBinError::into_binrw(e, pos))
            }
        }
    };
}
