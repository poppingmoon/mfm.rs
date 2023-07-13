use std::fmt::Display;

use crate::util::stringify_tree;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Node {
    Block(Block),
    Inline(Inline),
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Block(block) => write!(f, "{}", block),
            Node::Inline(inline) => write!(f, "{}", inline),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Block {
    Quote(Quote),
    Search(Search),
    CodeBlock(CodeBlock),
    MathBlock(MathBlock),
    Center(Center),
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Quote(quote) => write!(f, "{}", quote),
            Block::Search(search) => write!(f, "{}", search),
            Block::CodeBlock(code_block) => write!(f, "{}", code_block),
            Block::MathBlock(math_block) => write!(f, "{}", math_block),
            Block::Center(center) => write!(f, "{}", center),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Quote(pub Vec<Node>);

impl Display for Quote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let contents = stringify_tree(self.0.clone());
        contents
            .split('\n')
            .enumerate()
            .map(|(i, line)| {
                if i == 0 {
                    write!(f, "> {}", line)
                } else {
                    write!(f, "\n> {}", line)
                }
            })
            .collect::<Result<Vec<()>, std::fmt::Error>>()?;
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Search {
    pub query: String,
    pub content: String,
}

impl Display for Search {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CodeBlock {
    pub code: String,
    pub lang: Option<String>,
}

impl Display for CodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(lang) = &self.lang {
            write!(f, "```{}\n{}\n```", lang, self.code)
        } else {
            write!(f, "```\n{}\n```", self.code)
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MathBlock {
    pub formula: String,
}

impl Display for MathBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\\[\n{}\n\\]", self.formula)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Center(pub Vec<Inline>);

impl Display for Center {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<center>\n")?;
        self.0
            .iter()
            .map(|inline| write!(f, "{}", inline))
            .collect::<Result<Vec<()>, std::fmt::Error>>()?;
        write!(f, "\n</center>")
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Inline {
    UnicodeEmoji(UnicodeEmoji),
    EmojiCode(EmojiCode),
    Bold(Bold),
    Small(Small),
    Italic(Italic),
    Strike(Strike),
    InlineCode(InlineCode),
    MathInline(MathInline),
    Mention(Mention),
    Hashtag(Hashtag),
    Url(Url),
    Link(Link),
    Fn(Fn),
    Plain(Plain),
    Text(Text),
}

impl Display for Inline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Inline::UnicodeEmoji(unicode_emoji) => write!(f, "{}", unicode_emoji),
            Inline::EmojiCode(emoji_code) => write!(f, "{}", emoji_code),
            Inline::Bold(bold) => write!(f, "{}", bold),
            Inline::Small(small) => write!(f, "{}", small),
            Inline::Italic(italic) => write!(f, "{}", italic),
            Inline::Strike(strike) => write!(f, "{}", strike),
            Inline::InlineCode(inline_code) => write!(f, "{}", inline_code),
            Inline::MathInline(math_inline) => write!(f, "{}", math_inline),
            Inline::Mention(mention) => write!(f, "{}", mention),
            Inline::Hashtag(hashtag) => write!(f, "{}", hashtag),
            Inline::Url(url) => write!(f, "{}", url),
            Inline::Link(link) => write!(f, "{}", link),
            Inline::Fn(fn_) => write!(f, "{}", fn_),
            Inline::Plain(plain) => write!(f, "{}", plain),
            Inline::Text(text) => write!(f, "{}", text),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Simple {
    UnicodeEmoji(UnicodeEmoji),
    EmojiCode(EmojiCode),
    Text(Text),
}

impl Display for Simple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Simple::UnicodeEmoji(unicode_emoji) => write!(f, "{}", unicode_emoji),
            Simple::EmojiCode(emoji_code) => write!(f, "{}", emoji_code),
            Simple::Text(text) => write!(f, "{}", text),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UnicodeEmoji {
    pub emoji: String,
}

impl Display for UnicodeEmoji {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.emoji)
    }
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EmojiCode {
    pub name: String,
}

impl Display for EmojiCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ":{}:", self.name)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Bold(pub Vec<Inline>);

impl Display for Bold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "**")?;
        self.0
            .iter()
            .map(|inline| write!(f, "{}", inline))
            .collect::<Result<Vec<()>, std::fmt::Error>>()?;
        write!(f, "**")
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Small(pub Vec<Inline>);

impl Display for Small {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<small>")?;
        self.0
            .iter()
            .map(|inline| write!(f, "{}", inline))
            .collect::<Result<Vec<()>, std::fmt::Error>>()?;
        write!(f, "</small>")
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Italic(pub Vec<Inline>);

impl Display for Italic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<i>")?;
        self.0
            .iter()
            .map(|inline| write!(f, "{}", inline))
            .collect::<Result<Vec<()>, std::fmt::Error>>()?;
        write!(f, "</i>")
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Strike(pub Vec<Inline>);

impl Display for Strike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "~~")?;
        self.0
            .iter()
            .map(|inline| write!(f, "{}", inline))
            .collect::<Result<Vec<()>, std::fmt::Error>>()?;
        write!(f, "~~")
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InlineCode {
    pub code: String,
}

impl Display for InlineCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}`", self.code)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MathInline {
    pub formula: String,
}

impl Display for MathInline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r"\({}\)", self.formula)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Mention {
    pub username: String,
    pub host: Option<String>,
    pub acct: String,
}

impl Display for Mention {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.acct)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Hashtag {
    pub hashtag: String,
}

impl Display for Hashtag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.hashtag)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Url {
    pub url: String,
    pub brackets: bool,
}

impl Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.brackets {
            write!(f, "<{}>", self.url)
        } else {
            write!(f, "{}", self.url)
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Link {
    pub url: String,
    pub silent: bool,
    pub children: Vec<Inline>,
}

impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.silent {
            write!(f, "?[")?;
        } else {
            write!(f, "[")?;
        }
        self.children
            .iter()
            .map(|inline| write!(f, "{}", inline))
            .collect::<Result<Vec<()>, std::fmt::Error>>()?;
        write!(f, "]({})", self.url)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Fn {
    pub name: String,
    pub args: Vec<(String, Option<String>)>,
    pub children: Vec<Inline>,
}

impl Display for Fn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$[{}", self.name)?;
        let mut args = self.args.iter();
        if let Some((name, value)) = args.next() {
            write!(f, ".")?;
            if let Some(value) = value {
                write!(f, "{name}={value}")?;
            } else {
                write!(f, "{name}")?;
            }
        }
        for (name, value) in args {
            if let Some(value) = value {
                write!(f, ",{name}={value}")?;
            } else {
                write!(f, ",{name}")?;
            }
        }
        write!(f, " ")?;
        self.children
            .iter()
            .map(|inline| write!(f, "{}", inline))
            .collect::<Result<Vec<()>, std::fmt::Error>>()?;
        write!(f, "]")
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Plain(pub Vec<Text>);

impl Display for Plain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<plain>\n{}\n</plain>", self.0[0])
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Text {
    pub text: String,
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl From<Text> for Node {
    fn from(text: Text) -> Self {
        Node::Inline(Inline::Text(text))
    }
}

impl From<Text> for Inline {
    fn from(text: Text) -> Self {
        Inline::Text(text)
    }
}

impl From<Text> for Simple {
    fn from(text: Text) -> Self {
        Simple::Text(text)
    }
}

impl TryFrom<Node> for Text {
    type Error = ();

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        if let Node::Inline(Inline::Text(text)) = node {
            Ok(text)
        } else {
            Err(())
        }
    }
}

impl TryFrom<Inline> for Text {
    type Error = ();

    fn try_from(inline: Inline) -> Result<Self, Self::Error> {
        if let Inline::Text(text) = inline {
            Ok(text)
        } else {
            Err(())
        }
    }
}

impl TryFrom<Simple> for Text {
    type Error = ();

    fn try_from(inline: Simple) -> Result<Self, Self::Error> {
        if let Simple::Text(text) = inline {
            Ok(text)
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ValueOrText<T> {
    Value(T),
    Text(Text),
}
