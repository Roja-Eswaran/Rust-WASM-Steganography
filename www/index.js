import * as wasm from "wasm-ppm";

import { memory } from "wasm-ppm/wasm_ppm_bg";

var message="universe";
var global_data_array;
document.getElementById('btn-update').addEventListener(
	'click',
	function() {
		var txtArea = document.getElementById('text-input');
		var inputText = txtArea.value;
		message=inputText;
		let encode_img_ptr=wasm.encode_from_file(message,global_data_array);
		console.log("test");
		let bytesFromRust = new Uint8Array(
			memory.buffer,
			encode_img_ptr,
			global_data_array.length);

			if(bytesFromRust[0]==0){
				console.log("error_iruku");
				alert("There exists error in parsing PPM_Header");
				return;
				
			}

			if(bytesFromRust[0]==1){
				console.log("error_iruku");
				alert("MESSAGE SIZE IS TOO BIG TO ENCODE");
				return;
			}
	
				// now let's go back and stuff the ppm
				// into the javascript
				let blob = new Blob(
					[bytesFromRust],
					{type: 'image/x-portable-pixmap'});
	
				// stuff these bytes into the
				// img tag on our page
				const url = window.URL.createObjectURL(blob);
	
				const img = document.getElementById('img-ppm');
				img.src = url;
	
		  // conceptually, what we are doing is
		  // instead of stuffing the blob, which contains our
		  // ppm data into an image tag, we are going to
		  // create 'temporary' link, that download that data
		  // and then we are going to force the browser to
		  // click the the link, progmatically, and then it shows
		  // up as a download
		  const tempLink = document.createElement('a');
		  tempLink.style.display = 'none';
		  tempLink.href = url;
		  tempLink.setAttribute('download', "encode.ppm");
	
		  if (typeof tempLink.download === 'undefined') {
			tempLink.setAttribute('target', '_blank');
		  }
	
		  // add the temporary link to the document itself
		  document.body.appendChild(tempLink);
		  
		  // now "click" it
		  tempLink.click();
	
		  // now remove the link from the document
		  document.body.removeChild(tempLink);
	
		  // this is some firefox hack
		  setTimeout(() => {
			window.URL.revokeObjectURL(url);
		  }, 100);
	

	},
	false
);
const setImage = (imageBlob) => {


	// we need to get an arrayBuffer
	// that we can then convert to a Uint8Array
	// which we can then pass straight through to rust
	imageBlob.arrayBuffer().then(
		buff => {
			console.log(buff);

			let byteArray = new Uint8Array(buff);

	  let imageLength = byteArray.length;
	  global_data_array=byteArray;

      // we need to know the length (size?) of the image
      // because it's going to be stored in memory
      // and we need to be able to slice out that chunk
      // of memory

	
      }
	);
}

// grab the file from the browser when the user uploads it
// we want the file as an array of bytes
document.getElementById('file-input').addEventListener(
	'change',
	function() {
		var reader = new FileReader();
		var file = this.files[0];

		// async stuff
		// run this function when the reader has fired
		// the online event
		reader.onload = function() {
			var data = new Blob(
				[reader.result],
				{type: 'image/ppm'}
			);

			this.value = '';

			console.log(data);

			setImage(data);
		};

		// actually read the file in
		reader.readAsArrayBuffer(file);
	},
	false
);





document.getElementById('btn-update_1').addEventListener(
	'click',
	function() {
		var decode_message=wasm.decode_from_file(global_data_array);
		
		var string_2="error";
		var string_3="no";
		if(!string_3.localeCompare(decode_message)){
			console.log("No message");
			alert("No secret message is found in the file");
			return;
		}
		else if(!string_2.localeCompare(decode_message)){
			console.log("Error in decoding the message");
			alert("Error in decoding the message");
			return;
		}
		
		var outputPre = document.getElementById('output-pre');
		outputPre.textContent = decode_message.slice(0,decode_message.length-1);
		   var text = decode_message.slice(0,decode_message.length-1); 
		   var filename = "Decoded_Message.txt"; 

		   download(filename,text); 
	},
	false
);



function download(file,text) { 
  
	//creating an invisible element 
	var element = document.createElement('a'); 
	element.setAttribute('href','data:text/plain;charset=utf-8,'+ encodeURIComponent(text)); 
	element.setAttribute('download',file); 

	//the above code is equivalent to 
	// <a href="path of file" download="file name"> 

	document.body.appendChild(element); 

	//onClick property 
	element.click(); 

	document.body.removeChild(element); 
} 
