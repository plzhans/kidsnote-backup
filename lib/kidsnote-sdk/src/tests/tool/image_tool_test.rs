//#[cfg(tests)]
mod image_tool_test {

    #[test]
    fn wrap_text_test() {
        let text = "ㅠㅠ 오늘은 꼭 등원하려하는데.. 이제가 어제 못자서 그런지 안일어나네요.. 오후에 가도 괜찮은걸까요..?";
        let text2 = "ㅠㅠ오늘은꼭등원하려하는데..이제가어제못자서그런지안일어나네요..오후에가도괜찮은걸까요..?";
        
        let text_next = crate::tool::image_tool::ImageTool::wrap_text(text, 10);
        let text_next2 = crate::tool::image_tool::ImageTool::wrap_text(text2, 10);

        println!("{}", text_next);
        println!("");
        println!("{}", text_next2);
    }

}