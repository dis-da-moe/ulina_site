use yew::prelude::*;

pub fn navbar() -> Html{
    html!{
    <>
    <nav class="navbar navbar-light navbar-expand-md ulina-navbar" style="height: 57px;margin-top: 5px;">
        <div class="container-fluid"><button data-bs-toggle="collapse" class="navbar-toggler" data-bs-target="#navcol-1" style="color: rgb(255,255,255);border-style: none;border-color: rgb(255,255,255);"><span class="visually-hidden">{"Toggle navigation"}</span><span class="navbar-toggler-icon" style="color: rgb(255,255,255);"></span></button>
            <div class="collapse navbar-collapse fs-3" id="navcol-1">
                <ul class="navbar-nav">
                    <li class="nav-item text-center d-md-flex align-items-md-center nav-ulina"><a class="nav-link active" href="/" style="color: rgba(255,255,255,0.9);border-right-width: 0px;border-right-style: none;">{"Home"}</a></li>
                    <li class="nav-item text-center nav-ulina"><a class="nav-link active" href="/about" style="color: rgba(255,255,255,0.9);">{"About Ulina"}</a></li>
                    <li class="nav-item text-center nav-ulina"><a class="nav-link" href="/join" style="color: rgb(255,255,255);">{"How To Join"}</a></li>
                    <li class="nav-item text-center nav-ulina"><a class="nav-link" href="/tools/map" style="color: rgb(255,255,255);">{"Map"}</a></li>
                    <li class="nav-item text-center nav-ulina"><a class="nav-link" href="/tools/nations" style="color: rgb(255,255,255);">{"Nations"}</a></li>
                    <li class="nav-item text-center"><a class="nav-link" href="/tools/time" style="color: rgb(255,255,255);">{"Time"}</a></li>
                </ul>
            </div>
        </div>
    </nav>
    <script src="/assets/bootstrap/js/bootstrap.min.js"></script>
    </>
    }
}

#[macro_export]
macro_rules! navbar {
    () => {
        crate::components::navbar::navbar()    
    };
}