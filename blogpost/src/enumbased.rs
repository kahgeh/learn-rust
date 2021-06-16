enum State {
    Draft,
    PendingReview,
    Published
}

impl State {
    pub fn request_review(self)-> State{
        match self {
            State::Draft => State::PendingReview,
            _ => self,
        }
    }

    pub fn approve(self)->State{
        match self {
            State::PendingReview => State::Published,
            _ => self,
        }
    }
}

pub struct Post {
    state: Option<State>,
    content: String,
}

impl Post {
    pub fn new()->Post{
        Post {
            state : Some(State::Draft),
            content: String::new(),
        }
    }

    pub fn request_review(&mut self){
        let state = self.state.take().unwrap();
        self.state =  Some(state.request_review());
    }

    pub fn approve(&mut self){
        let state = self.state.take().unwrap();
        self.state =  Some(state.approve());
    }

    pub fn add_text(&mut self, text: &str){
        self.content.push_str(text);
    }

    pub fn content(&self)-> &str{
        match self.state {
            Some(State::Published) => self.content.as_ref(),
            _ => ""
        }
    }
}