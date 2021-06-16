mod blog;
use blog::Post;
// mod gui;
// use gui::{Button, Draw, Screen, SelectBox};

fn main() {
    /* 1. for draw screen
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("예"),
                    String::from("아니오"),
                    String::from("모름"),
                ],
            }),
            Box::new(Button {
                width: 75,
                height: 10,
                label: String::from("확인"),
            }),
        ],
    };

    screen.run();
    */

    // 2. blog

    // 1) 블로그의 새 초고를 작성하기
    let mut post = Post::new();

    // 2) 텍스트를 추가할 수 있는 함수를 제공
    post.add_text("나는 오늘 점심으로 샐러드를 먹었다.");

    // 3) 블로그의 리뷰를 요청하기
    let post = post.request_review();
    // 아직 게재가 승인된 상태가 아니므로 빈 문자열을 리턴해야 한다

    // 4) 포스트가 승인을 받는다면 앞서 입력했던 text와 같아야 할 것
    let post = post.approve();
    assert_eq!("나는 오늘 점심으로 샐러드를 먹었다.", post.content());
}
