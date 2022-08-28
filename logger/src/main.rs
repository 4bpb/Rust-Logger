use logger::log;
mod logger;


fn main() {
    log("This is a Test", "err");
    log("This is a Test", "init");
    log("This is a Test", "ok");
    log("This is a Test", "");
    
    //log("This is a Test", "rain");
}


