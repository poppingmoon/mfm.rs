mod to_string {
    #[test]
    fn basic() {
        let input = "before
<center>
Hello $[tada everynyan! 🎉]

I'm @ai, A bot of misskey!

https://github.com/syuilo/ai
</center>
after";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);
    }

    #[test]
    fn single_node() {
        let input = "$[tada Hello]";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), "$[tada Hello]");
    }

    #[test]
    fn quote() {
        let input = "
> abc
>
> 123";
        assert_eq!(
            mfm::to_string(mfm::parse(input).unwrap()),
            "> abc\n> \n> 123"
        );
    }

    #[test]
    fn search() {
        let input = "MFM 書き方 123 Search";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);
    }

    #[test]
    fn block_code() {
        let input = "```\nabc\n```";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);
    }

    #[test]
    fn math_block() {
        let input = "\\[\ny = 2x + 1\n\\]";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);
    }

    #[test]
    fn center() {
        let input = "<center>\nabc\n</center>";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);
    }

    #[test]
    fn emoji_code() {
        let input = ":abc:";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);
    }

    #[test]
    fn unicode_emoji() {
        let input = "今起きた😇";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);
    }

    #[test]
    fn big() {
        let input = "***abc***";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), "$[tada abc]");
    }

    #[test]
    fn bold() {
        let input = "**abc**";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);
    }

    #[test]
    fn small() {
        let input = "<small>abc</small>";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);
    }

    #[test]
    fn italic_tag() {
        let input = "<i>abc</i>";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);
    }

    #[test]
    fn strike() {
        let input = "~~foo~~";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);
    }

    #[test]
    fn inline_code() {
        let input = "AiScript: `#abc = 2`";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);
    }

    #[test]
    fn math_inline() {
        let input = "\\(y = 2x + 3\\)";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);
    }

    #[test]
    fn hashtag() {
        let input = "a #misskey b";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);
    }

    #[test]
    fn link() {
        let input = "[Ai](https://github.com/syuilo/ai)";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);
    }

    #[test]
    fn silent_link() {
        let input = "?[Ai](https://github.com/syuilo/ai)";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);
    }

    #[test]
    fn fn_() {
        let input = "$[tada Hello]";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);
    }

    #[test]
    fn fn_with_arguments() {
        let input = "$[spin.speed=1s,alternate Hello]";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);
    }

    #[test]
    fn plain() {
        let input = "a\n<plain>\nHello\nworld\n</plain>\nb";
        println!("{:?}", mfm::parse(input).unwrap());
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);
    }

    #[test]
    fn one_line_plain() {
        let input = "a\n<plain>Hello</plain>\nb";
        assert_eq!(
            mfm::to_string(mfm::parse(input).unwrap()),
            "a\n<plain>\nHello\n</plain>\nb"
        );
    }

    #[test]
    fn preserve_url_brackets() {
        let input = "https://github.com/syuilo/ai";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);

        let input = "<https://github.com/syuilo/ai>";
        assert_eq!(mfm::to_string(mfm::parse(input).unwrap()), input);
    }
}
