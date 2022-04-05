#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ArturiaMinilabMkiiKeys {
    Key1,
    Key2,
}

impl ArturiaMinilabMkiiKeys {
    pub fn as_str(&self) -> &'static str {
        match self {
            ArturiaMinilabMkiiKeys::Key1 => "C:\\audio1.mp3",
            ArturiaMinilabMkiiKeys::Key2 => "C:\\audio2.mp3",
        }
    }
}
