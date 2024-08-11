const token = getCookie("token");
const username = getCookie("user");       


function getCookie(name) {
    const value = `; ${document.cookie}`;
    const parts = value.split(`; ${name}=`);
    if (parts.length === 2) return parts.pop().split(';').shift();
    }



function deleteCookie(name) {
    document.cookie = `${name}=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;`;
}


document.getElementById('logoutButton').addEventListener('click', function() {
    console.log("logout");
    deleteCookie('username');
    deleteCookie('token');
  
    window.location.href = '/dash/login';
});



// ----------------- night and light mode ------------------//
const profileArea = document.getElementById("profile-area");
const pagesArea = document.getElementById("pages-info");
const footerArea = document.getElementById("footer");
theme();
document.addEventListener('DOMContentLoaded', (event) => {
    const nightCheckbox = document.getElementById('night-checkbox');

    nightCheckbox.addEventListener('change', function() {
        if (nightCheckbox.checked) {
            // lightMode();
       
            deleteCookie("theme");
            document.cookie = "theme=light; path=/; SameSite = Strict ";

            theme();
            
            //set lightmode cookie
            //delete night cookie
        }
        else{
            // nightMode();
            
            deleteCookie("theme");
            document.cookie = "theme=night; path=/; SameSite = Strict";
            // set night cookie
            // delete light cookie
            theme();
        } 
        });
    });
// ----------------- night and light mode ------------------//
function theme(){
    const nightCheckbox = document.getElementById('night-checkbox');
    let theme = getCookie("theme");
    if(theme === "light"){
        nightCheckbox.checked = true;
        document.body.style = "color: #282828";
        profileArea.style = "background-color: #e8e8e8; color: #282828";
        pagesArea.style = "background-color: #e8e8e8; color: #282828";
        footerArea.style = "background-color: #e8e8e8; color: #585858"
    }
    else if(theme === "night"){
        nightCheckbox.checked = false;
        document.body.style = "color: #f2f2f2; background-color: #282828";
        profileArea.style = "background-color: #383838; color: #f2f2f2";
        pagesArea.style = "background-color: #383838; color: #f2f2f2";
        footerArea.style = "background-color: #383838; color: #a8a8a8";
    }
}


document.getElementById("title-container").addEventListener("click", function() {

     window.location.href = '/dash';
});

const userDisplay = document.getElementById("username");
    userDisplay.textContent = `${username}`