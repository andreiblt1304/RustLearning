pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost { content: String::new() }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { content: self.content }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content
        }
    }
}

// pub struct Post {
//     state: Option<Box<dyn State>>,
//     content: String,
//     approvals: u32,
// }

// impl Post {
//     pub fn new() -> Post {
//         Post {
//             state: Some(Box::new(Draft {})),
//             content: String::new(),
//             approvals: 0,
//         }
//     }

//     pub fn add_text(&mut self, text: &str) {
//         &self.content.push_str(text);
//     }

//     pub fn content(&self) -> &str {
//         self.state.as_ref().unwrap().content(self)      
//         //returns reference instead of ownership of the value
//     }

//     pub fn request_review(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.request_review())
//         }
//     }

//     pub fn get_count(&self) -> u32 {
//         self.approvals
//     }

//     pub fn approve(&mut self) {
//         self.approvals += 1;
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.approve());
//         }
//     }

//     pub fn reject(&mut self) {
//         if self.approvals >= 2 {
//             if let Some(s) = self.state.take() {
//                 self.state = Some(s.reject())
//             }
//         }
//     }
// }

// trait State {
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
//     fn approve(self: Box<Self>) -> Box<dyn State>;
//     fn content<'a>(&'a self, post: &'a Post) -> &str { "" }
//     fn reject(self: Box<Self>) -> Box<dyn State>;
// }

// struct Draft {}

// impl State for Draft {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview {})
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
// }

// struct PendingReview {}

// impl State for PendingReview {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Published {})
//     }

//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Draft {})
//     }
// }

// struct Published {}

// impl State for Published {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn content<'a>(&'a self, post: &'a Post) -> &str {
//         &post.content
//     }

//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
// }