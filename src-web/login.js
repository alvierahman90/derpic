const apiUrl = new URL(window.location.href);
apiUrl.path = '/i';
const apiUrl = apiUrl.href;

const loginGrid = document.getElementById("login-grid");

//---------showpass password-----------//
function showPWD() {
    console.log("showpwd");
    const tokenInput = document.getElementById("token");
    const showPwdButton = document.getElementById("showPWD");
    if (tokenInput.type === "password") {
        tokenInput.type = "text";
        showPwdButton.innerHTML = "<i class='glyphicon glyphicon-eye-close' style='font-size: 18px;'></i>"

    } else {
        tokenInput.type = "password";
        showPwdButton.innerHTML = "<i class='glyphicon glyphicon-eye-open' style='font-size: 18px;'></i>"
    }
  } 
// -----------------

//--------------login functionality-----------------//

document.getElementById("login-info").addEventListener("submit", function(event){
    event.preventDefault();
    console.log("login");
    const username = document.getElementById("username").value;
    const token = document.getElementById("token").value;

    if(!username || !token){
        alert("Please enter both username and token");
        return;
    }

    document.cookie = `token=${token}; path=/; SameSite = Strict`;
    document.cookie = `user=${username}; path=/;SameSite = Strict`;

    const requestOptions = {
        method: 'GET',
        headers: {
          'Accept': 'application/json; charset=utf-8',
          'X-Derpic-Username': `${username}`,
          'X-Derpic-Token': `${token}`
          // 'origin': 'any-random-text'
        },
      };
      
      fetch(apiUrl, requestOptions)
        .then(response => {
          if (!response.ok) {

          const password = document.getElementById('token');
          const username = document.getElementById("username");
          const popup = document.createElement("p");
          // tokenInput.type = "text";
          popup.style = "color: darkred; text-align:center; font-weight: 700";
          popup.classList = "grid-col-span-2"
          popup.textContent = "- Invalid token or username -";
          loginGrid.appendChild(popup);
          password.style = "border: 2px solid darkred;"
          password.value = "";
          username.style =  "border: 2px solid darkred;"
          
         

          // tokenInput.value = "";
          password.disabled = 1;

          setTimeout(() => {
            password.style = "";
            username.style = "";
            loginGrid.removeChild(popup);
          // tokenInput.value = "";
          // tokenInput.type = "password";
          password.disabled = 0;
          }, 1800);

          throw new Error('Unauthorized: The token is invalid or expired.');

       
          }
          return response.json();
        })
        .then(data => {
            console.log(data);
            window.location.href = "/dash";
        });
        })
        .catch(error => {
          console.error('Error:', error);
       

});


// ---------------------------------------------

