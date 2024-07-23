
const token = getCookie("token");
const username = getCookie("user");                      //sets the token and username from cookies

const apiUrl = new URL(window.location.href);
apiUrl.path = '/i';
const apiUrl = apiUrl.href;

const apiKey = `${token}`;

const dropArea = document.getElementById("drop-area");    // setting global areas and variables
const inputFile = document.getElementById("input-img");
const imageView = document.getElementById("img-view");

let selectedItem = null;
let slug = "";
let liveSlug = "";


loadGallery();      // calls load gallery before anything.

// ------------ API Fetch GET to load gallery -----------
function loadGallery(){
    const requestOptions = {
    method: 'GET',
    headers: {
        'X-Derpic-Token': `${apiKey}`,
        // 'origin': 'any-random-text'
    },
    };

    fetch(apiUrl, requestOptions)
    .then(response => {
        if (!response.ok) {
        window.location.href = "/dash/login";
        throw new Error('Network response was not ok');
       
        }
        return response.json();
    })
    .then(data => {
        document.getElementById('galleryGrid').innerHTML = ""
        // let slug = data[0].slug;
        // let id = data[0].id;
        // console.log(data.length);
        // console.log(slug, id);
        for(let i = 0; i < data.length; i++){
            imgDataStorage = `${apiUrl}/${data[i].slug}`     //using the token iterates through the data and displays the images in the gallery hosted from the backend
            const img = document.createElement('img');
            img.src = imgDataStorage;
            img.className = "galleryImg";
            const cell = document.createElement('div');
            cell.className = 'grid-item';
            cell.id = `grid-item-slug-${data[i].slug}`;
            cell.appendChild(img);
            document.getElementById('galleryGrid').appendChild(cell);

        }
    })
    .catch(error => {
        console.error('Error:', error);
    });
}


//------------------ API fetch POST to upload images ------


function uploadImageAPI(){

        const file = inputFile.files[0];
        console.log(`Selected file: ${file.name}, type: ${file.type}, size: ${file.size} bytes`);
        if (file) {

            // const formData = new FormData();
            // formData.append('file',file);

            const requestOptions = {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'X-Derpic-Token': `${apiKey}`,
                'Content-Type': 'application/octet-stream'

            },
            body: file
            };

            fetch(apiUrl, requestOptions)
            .then(response => {
                if (!response.ok) {
                throw new Error('Network response was not ok');
                }
                return response.json();
            })
            .then(data => {
                console.log(data);
                loadGallery();
                localStorage.removeItem('imageData');
                document.getElementById('uploadButton').disabled = true;
                document.getElementById('input-img').value = '';
                resetDropArea();
                displayMetadata();
                const metadataElement = document.getElementById('metadata');
                metadataElement.innerHTML = "File Name:<br>File Size:<br>File Type:<br>Image Width:<br>Image Height:";
            })
            .catch(error => {
                console.error('Error:', error);
            });
        }
        else{
            console.error("No file selected");
        }
    }




//----------------- API fetch DELETE to remove images -------------

function deleteImageAPI(){

    if (!slug) {
        console.error('No slug available for deletion.');
        return;
    }
            
    const requestOptions = {
        method: 'DELETE',
        headers: {
            'X-Derpic-Token': `${apiKey}`,
        },
        };
    
        fetch(`${apiUrl}/${slug}`, requestOptions)
        .then(response => {
            if (!response.ok) {
            throw new Error('Network response was not ok');
            }
            return response.json();
        })
        .then(data => {
            document.getElementById('deleteButton').disabled = true;
            document.getElementById('metadata').textContent = "";
            slug = "";
            loadGallery();
        })
    .catch(error => {
        console.error('Error:', error);
    });
}



//----------Displays the file info on input-----------



function clearDisplayCopyImg(){
    const gridContainer = document.querySelector('.copy-area');
    gridContainer.innerHTML = "";
}

inputFile.addEventListener("change", uploadImage);


document.getElementById('input-img').addEventListener('change', function(event) {
    const file = event.target.files[0];
    if (file) {
        document.getElementById('uploadButton').disabled = false;
        const fileInfo = document.getElementById('metadata');
        
      
        fileInfo.textContent = `File Name: ${file.name}\nFile Size: ${file.size} bytes\nFile Type: ${file.type}`;
        
        const reader = new FileReader();
        reader.onload = function(e) {
            const img = new Image();
            img.onload = function() {
                fileInfo.textContent += `\nImage Width: ${img.width}px\nImage Height: ${img.height}px`;
            };
            img.src = e.target.result;
            localStorage.setItem("imageData", e.target.result);
            
          
        };
        reader.readAsDataURL(file);
    }
});

// ------------uploadImage function sets image as background--------

function uploadImage(){
    selectedItem = null;
    slug = "";
    
    let imgLink = URL.createObjectURL(inputFile.files[0]);
    imageView.textContent = "";
    const pic = document.createElement("div");
    pic.className = "mainPic";
    pic.id = "mainPic";
    let picture = document.createElement("img");
    picture.src = imgLink;
    pic.appendChild(picture);
    imageView.appendChild(pic);
    imageView.style.border = 0;
    
    clearDisplayCopyImg();
}

// ---------- display metadata (file info) when upload -------
function displayMetadata(tags) {
    const metadataElement = document.getElementById('metadata');
    metadataElement.textContent = JSON.stringify(tags, null, 2);
}

// ---------- async function to get selcted img blob and display info such as slug -------------
async function getImageFile() {
    const imgElement = document.getElementById('imgElement');
    const metadataElement = document.getElementById('metadata');

    if (imgElement && imgElement.src) {
        const imgUrl = imgElement.src;

        try {
            const response = await fetch(imgUrl);
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            const blob = await response.blob();

            const urlParts = imgUrl.split('/');
            const filename = urlParts[urlParts.length - 1];

            const file = new File([blob], filename, { type: blob.type });

            metadataElement.textContent = `Slug: ${file.name}\nFile Size: ${file.size} bytes\nFile Type: ${file.type}`;
        } catch (error) {
            console.error('Error fetching image:', error);
            metadataElement.textContent = 'Error fetching image';
        }
    } else {
        metadataElement.textContent = '';
        if (imgElement) {
            imgElement.src = '';
        }
    }
}



// -------------- Nightmode/lightmode functionality--------------//
const leftArea = document.getElementById("left-area");
const rightArea = document.getElementById("right-area");
const inputArea = document.getElementById("input-area");
const gridItem = document.getElementById("grid-item");
const profileArea = document.getElementById("profile-area");

document.addEventListener('DOMContentLoaded', (event) => {
    const nightCheckbox = document.getElementById('night-checkbox');

    nightCheckbox.addEventListener('change', function() {
        if (nightCheckbox.checked) {
            lightMode();
        }
        else{
            nightMode();
        } 
        });
    });
// ----------------- night and light mode ------------------//

function nightMode(){
    document.body.style = "color: #f2f2f2; background-color: #282828";
    leftArea.style = "background-color: #383838";
    rightArea.style = "background-color: #282828; border-color: #383838";
    inputArea.style = "background-color: #383838 ;color: #f2f2f2;";
    profileArea.style = "background-color: #383838; color: #f2f2f2"
}

function lightMode(){
    console.log("light");
    document.body.style = "color: #282828";
    leftArea.style = "background-color: #e8e8e8";
    rightArea.style = "background-color: #f2f2f2";                              // setting dark and light mode (super botch but cba to change)
    inputArea.style = "background-color: #e8e8e8 ;color: #282828;";
    profileArea.style = "background-color: #e8e8e8; color: #282828"
}

// --------------------------- DELETE FUNCTIONALITY--------//

// firstly we create a selected item function and class to apply css and gather info

    document.addEventListener('DOMContentLoaded', () => {
        const grid = document.getElementById('galleryGrid');
        const deleteBtn = document.getElementById('deleteButton');
        let selectedItem = null;
    
 
        grid.addEventListener('click', function(event) {
            if (event.target.classList.contains('grid-item') || event.target.closest('.grid-item')) {
                let previouslySelectedItem = document.querySelector('.grid-item.selected');
                selectedItem = event.target.closest('.grid-item');
                if (event.target.closest(".selected")) {
        
                    selectedItem.classList.remove('selected');
                    selectedItem = null;
                    document.getElementById('deleteButton').disabled = true;
                    clearSelectedPopup();
                    clearDisplayCopyImg();
                }
                else if (!event.target.closest(".selected")){
                    
                    if (previouslySelectedItem) {
                        previouslySelectedItem.classList.remove('selected');
                    }
                    selectedItem = event.target.closest('.grid-item');
                    selectedItem.classList.add('selected');
                    const slugId = selectedItem.id;
                    const parts = slugId.split("-");
                    slug = parts.slice(3).join("-");
                    console.log(slug);
                    selectedPopup();
                    getImageFile();
                    clearDisplayCopyImg();
                    displayCopyImg();
                    document.getElementById('deleteButton').disabled = false;
                    
                }



                // ---------- send the selected photo to the img-veiw window
              
                // ----------
            }
        });

// ----------- selected item popup -------------//

function selectedPopup(){
    document.getElementById('uploadButton').disabled = true;
    let imgElement = selectedItem.children[0];
    let imgLink = imgElement.getAttribute('src');
    imageView.textContent = "";
    imageView.style.border = 0;
    const pic = document.createElement("div");
    pic.className = "mainPic";
    pic.id = "mainPic";
    let picture = document.createElement("img");
    picture.id = "imgElement";
    picture.src = imgLink;
    pic.appendChild(picture);
    
    imageView.appendChild(pic);
    
}
// -------------- selected popup from slug -------------
function selectedPopupFromSlug(liveSlug){
    imageView.textContent = "";
    imageView.style.border = 0;
    const pic = document.createElement("div");
    pic.className = "mainPic";
    pic.id = "mainPic";
    let picture = document.createElement("img");
    picture.id = "imgElement";
    picture.src = liveSlug;
    pic.appendChild(picture);
    imageView.appendChild(pic);
}
//--------------- clear selected popup -----------------

function clearSelectedPopup(){
    resetDropArea();
    getImageFile();
}

//-------copy img URL and img URL editor-------

function displayCopyImg(){
    // clearDisplayCopyImg();
    const gridContainer = document.querySelector('.copy-area');
    //-- set labels --
    const customLabel = document.createElement("label");
    const flipvLabel = document.createElement("label");
    const fliphLabel = document.createElement("label");
    customLabel.classList.add("customLabel");
    fliphLabel.classList.add("fliphLabel");
    flipvLabel.classList.add("flipvLabel");
   
   

    //-- set inputs --
    const custom = document.createElement("input");
    const copyURL = document.createElement("button");
   
    //-- set type --
    custom.setAttribute('type', 'checkbox');
   
    //-- set min max --

   
    //-- set names --
  
    copyURL.innerHTML = "Copy";
    copyURL.setAttribute("onclick", "copyURL()")
    //-- set class names for styling --
    custom.id = "cus";
  
    copyURL.classList.add("Copybutton");
    gridContainer.appendChild(customLabel);
    //-- set spans --

  
    const customInnerLabel = document.querySelector(".customLabel");
   
    const customSpan = document.createElement("span");
    customSpan.classList.add("customSpan");
  
    
    customLabel.append(custom);
  
    customInnerLabel.append(customSpan);

    gridContainer.appendChild(copyURL);

    const customCheck = document.getElementById("cus");

    function updateCustom(){

        
        if(customCheck.checked){
            const gridContainer = document.querySelector('.copy-area');

            const rotation = document.createElement("input");
            const width = document.createElement("input");
            const height = document.createElement("input");
            const flipv = document.createElement("input");
            const fliph = document.createElement("input");

            rotation.setAttribute('type', 'number');
            width.setAttribute('type', 'number');
            height.setAttribute('type', 'number');
            flipv.setAttribute('type', 'checkbox');
            fliph.setAttribute('type', 'checkbox');

            rotation.setAttribute('min', '0');
            rotation.setAttribute('max', '360'); 
            rotation.setAttribute('step', '90'); 
        
            width.setAttribute('min', '5');
            width.setAttribute('max', '500'); 
        
            height.setAttribute('min', '5');
            height.setAttribute('max', '500'); 

            rotation.setAttribute('placeholder', 'Rotation');
            width.setAttribute('placeholder', 'Width %');
            height.setAttribute('placeholder', 'Height %');

            rotation.id= "rt";
            width.id="wd";
            height.id="hi";
            flipv.id="fv";
            fliph.id="fh";

            gridContainer.appendChild(rotation);
            gridContainer.appendChild(width);
            gridContainer.appendChild(height);
            
            gridContainer.appendChild(fliphLabel);
            gridContainer.appendChild(flipvLabel);
            
            const fliphInnerLabel = document.querySelector(".fliphLabel");
            const flipvInnerLabel = document.querySelector(".flipvLabel");
            const flipvSpan = document.createElement("span");
            const fliphSpan = document.createElement("span");
            flipvSpan.classList.add("flipvSpan");
            fliphSpan.classList.add("fliphSpan");
            flipvLabel.appendChild(flipv);
            fliphLabel.appendChild(fliph);
            fliphInnerLabel.appendChild(fliphSpan);
            flipvInnerLabel.appendChild(flipvSpan);

            const rotationCN = document.getElementById("rt");
            const widthCN = document.getElementById("wd");
            const heightCN = document.getElementById("hi");

            rotationCN.addEventListener("input", function(){
        
            if(rotationCN.value > 360) {
                rotationCN.value = 360;
            }
            });

            widthCN.addEventListener("input", function(){
        
                if(widthCN.value > 500) {
                    widthCN.value = 500;
                }
                });

            
            heightCN.addEventListener("input", function(){
        
            if(heightCN.value > 500) {
                heightCN.value = 500;
            }
            });
            
            const rotationLive = document.getElementById("rt");
            const widthLive = document.getElementById("wd");
            const heightLive = document.getElementById("hi");
            const flipvCB = document.getElementById("fv");
            const fliphCB = document.getElementById("fh");
            rotationLive.addEventListener("input", updateImgLive);
            widthLive.addEventListener("input", updateImgLive);
            heightLive.addEventListener("input", updateImgLive);
            fliphCB.addEventListener("input", updateImgLive);
            flipvCB.addEventListener("input", updateImgLive);
            }   
        else{
            selectedPopupFromSlug(`${apiUrl}/${slug}`);
            document.getElementById("rt")?.remove();
            document.getElementById("wd")?.remove();
            document.getElementById("hi")?.remove();
            document.getElementById("fv")?.remove();
            document.getElementById("fh")?.remove();
            flipvLabel.remove();
            fliphLabel.remove();
        }

    }
    custom.addEventListener('change', updateCustom);
    updateCustom();

    function updateImgLive(){
        const rotation = document.getElementById("rt");
        const width = document.getElementById("wd");
        const height = document.getElementById("hi");
        const flipvCB = document.getElementById("fv");
        const fliphCB = document.getElementById("fh");
        let originHeight = extractHeight();
        let originWidth = extractWidth();
        let rotationdeg = 0;
        let widthpx = calculateWidth(originWidth,100);
        let heightpx = calculateHeight(originHeight,100);
        let flipvBool;
        let fliphBool;

        const allowedValues = [0, 90, 180, 270, 360];
        const rotvalue = parseInt(rotation.value, 10);

        if(rotation.value != "" && allowedValues.includes(rotvalue)){
            rotationdeg = rotation.value;
        }
        else{
            // alert the user to pick a rotation between 0 and 360 (step of 90)
            // rotation.style = "border: 2px solid #d45500";
            // setTimeout(() => {
            //     rotation.style = "border: 3px solid #282828";
            // }, 1500);
        }
        if(width.value != "" && width.value > 5){
            widthpx = calculateWidth(originWidth,width.value);
         
            
        }
        else{
            // width.style = "border: 2px solid #d45500";
            // setTimeout(() => {
            //     width.style = "border: 3px solid #282828";
            // }, 1500);
           
        }
        if(height.value != "" && height.value > 5){
            heightpx = calculateHeight(originHeight,height.value);
           
        }
        else{
            // height.style = "border: 2px solid #d45500";
            // setTimeout(() => {
            //     height.style = "border: 3px solid #282828";
            // }, 1500);
        }

        if(fliphCB.checked){
            fliphBool = "true";
        }
        else{
            fliphBool = "false";
        }

        if(flipvCB.checked){
            flipvBool = "true";
        }
        else{
            flipvBool = "false";
        }

        liveSlug = `${apiUrl}/${slug}?rotation=${rotationdeg}&width=${widthpx}&height=${heightpx}&flipv=${flipvBool}&fliph=${fliphBool}`;
        selectedPopupFromSlug(liveSlug);
    }
}




// delete button function calls the deleteImageAPI and resets the area and selected item.
        deleteBtn.addEventListener('click', function() {
            if (selectedItem) {
                grid.removeChild(selectedItem);
                selectedItem = null;
                clearDisplayCopyImg();
                deleteImageAPI();
                resetDropArea();
            } else {
                alert('Please select an item to delete.');
            }
        });
     });
//---------------------- function  that resets the img-view area ------------------------------

    function resetDropArea(){
        var parentDiv = document.getElementById('img-view');
        var childDiv = document.getElementById('mainPic');
      
       
        if (parentDiv && childDiv) {
          parentDiv.removeChild(childDiv);
        } else {
          console.error('Parent or child div not found!');
        }
        imageView.innerHTML = "<i class='fa fa-photo' style='font-size: 150px';></i><p>Click here <br> to upload image</p>";
        imageView.style.border = "2px dashed #a8a8a8";
       
    

    }
//---------------



//--------- function to deal with cookies -------

function getCookie(name) {
    const value = `; ${document.cookie}`;
    const parts = value.split(`; ${name}=`);
    if (parts.length === 2) return parts.pop().split(';').shift();
    }
    

    console.log(token);
    console.log(username);

    const userDisplay = document.getElementById("username");
    userDisplay.textContent = `${username}`



// ------- logout button functionality ------

function deleteCookie(name) {
    document.cookie = `${name}=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;`;
}
  

document.getElementById('logoutButton').addEventListener('click', function() {
    console.log("logout");
    deleteCookie('username');
    deleteCookie('token');
  
    window.location.href = '/derpic/derpic-login/derpic-login.html';
});

//--------- adjustment of font size for info-area -------------

function adjustFontSize() {
    const container = document.getElementById('info-area');
    const text = document.getElementById('metadata');
    let containerWidth = container.clientWidth;
    
    let fontSize = containerWidth / 19; 
    text.style.fontSize = fontSize + 'px';
  }

  window.addEventListener('resize', adjustFontSize);
  window.addEventListener('load', adjustFontSize);
  //----------- generate url for image specs -------------

  // event listen for copy button

  // create image with url in format [https://i.dev.alv.cx/i/X65Cp2GsVaHMiaFjUxaEzA?rotation=90&width=50&height=50&flipv=true&fliph=false]
  // with info from client.

function copyURL(){
    const customCheck = document.getElementById("cus");
    if(customCheck.checked){

    
        const rotation = document.getElementById("rt");
        const width = document.getElementById("wd");
        const height = document.getElementById("hi");
        const flipvCB = document.getElementById("fv");
        const fliphCB = document.getElementById("fh");
        let rotationdeg;
        let widthpx;
        let heightpx;
        let flipvBool;
        let fliphBool;

        originHeight = extractHeight();
        originWidth = extractWidth();


        const allowedValues = [0, 90, 180, 270, 360];
        const rotvalue = parseInt(rotation.value, 10);

        if(rotation.value != "" && allowedValues.includes(rotvalue)){
            rotationdeg = rotation.value;
        }
        else{
            // alert the user to pick a rotation between 0 and 360 (step of 90)
            rotation.style = "border: 2px solid #d45500";
            setTimeout(() => {
                rotation.style = "border: 3px solid #282828";
            }, 1500);
        }
        if(width.value != "" && width.value > 5){
            widthpx = calculateWidth(originWidth,width.value);
        }
        else{
            width.style = "border: 2px solid #d45500";
            setTimeout(() => {
                width.style = "border: 3px solid #282828";
            }, 1500);
           
        }
        if(height.value != "" && height.value > 5){
            heightpx = calculateHeight(originHeight,height.value);
        }
        else{
            height.style = "border: 2px solid #d45500";
            setTimeout(() => {
                height.style = "border: 3px solid #282828";
            }, 1500);
        }
        if(fliphCB.checked){
            fliphBool = "true";
        }
        else{
            fliphBool = "false";
        }

        if(flipvCB.checked){
            flipvBool = "true";
        }
        else{
            flipvBool = "false";
        }

        if(widthpx && heightpx && rotationdeg){
        let copyURL = `${apiUrl}/${slug}?rotation=${rotationdeg}&width=${widthpx}&height=${heightpx}&flipv=${flipvBool}&fliph=${fliphBool}`;
        
        navigator.clipboard.writeText(copyURL).then(function() {
            alert("URL copied to clipboard");
        }, function(err){
            console.error("Could not copy text: ", err);
        });

        }   
        else{
            return;
        }
    }
    else{
        let copyURL = `${apiUrl}/${slug}`;
        navigator.clipboard.writeText(copyURL).then(function() {
            alert("URL copied to clipboard");
        }, function(err){
            console.error("Could not copy text: ", err);
        });
    }
}


// -------- event listner for width, height or rotation input --------

//-------------- turn the width and height into percentages ----------

// when an image is uploaded apply a class with the height and width infomation.
// then when you select the image it will extract the info from the class name.
// when assigning the width and height call a function to multiply the percentage by the height and width.

function extractHeight() {
    const img = new Image;
    img.src = `${apiUrl}/${slug}`

 
      
    const height = img.height;
  
    return height;
    

}

function extractWidth() {
    const img = new Image;
    img.src = `${apiUrl}/${slug}`

   
        
    const width = img.width;
    
    return width;
    
}

function calculateWidth(originWidth, widthIn){
    let percentageWidth = Math.ceil(originWidth * widthIn/100);
    return percentageWidth;
}
function calculateHeight(originHeight, heightIn){
    let percentageHeight = Math.ceil(originHeight * heightIn/100);
    return percentageHeight;
}

// needs a major overhaul but its a WIP!
