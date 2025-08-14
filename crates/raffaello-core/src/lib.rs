macro_rules! component_args {
    ($compname:ident, $($arg:ident => $type:ty)*) => {

    };
}

macro_rules! component {
    ($name:ty) => {
        
    };
}

trait Component {
    fn init();
    fn render();
}

component_args!(Paragraph, color => Color);

component!(Paragraph);

pub struct Draw {

}