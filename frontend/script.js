const ws = new WebSocket('ws://127.0.0.1:8080/ws');
const messages = document.getElementById('messages');
const input = document.getElementById('input');
const usernameInput = document.getElementById('username');
const fileInput = document.getElementById("fileInput");
const messageContainer = document.getElementById("message-container");

let username = "";

usernameInput.addEventListener('keypress', (e) => {
    if (e.key === 'Enter' && usernameInput.value.trim()) {
        username = usernameInput.value;
        ws.send(username);
        usernameInput.style.display = 'none';
        messageContainer.style.display = 'flex';
        input.focus();
    }
});

fileInput.addEventListener("change", () => {
    if (fileInput.files.length > 0) {
        const file = fileInput.files[0];
        const reader = new FileReader();

        reader.onload = () => {
            const fileMessage = JSON.stringify({ 
                type: "file", 
                name: file.name, 
                content: reader.result 
            });
            ws.send(fileMessage);
        };

        reader.onerror = () => {
            alert("Error reading file. Please try again.");
        };

        reader.readAsDataURL(file);
    }
});

ws.onmessage = (event) => {
    try {
        const data = JSON.parse(event.data);
        
        if (data.type === 'file') {
            const fileLink = document.createElement('a');
            fileLink.href = data.content;
            fileLink.download = data.name;
            fileLink.textContent = `📎 ${data.name}`;
            addMessage(`${data.sender}: `, data.sender === username, fileLink);
        } else if (data.type === 'message') {
            addMessage(`${data.sender}: ${data.content}`, data.sender === username);
        } else if (data.type === 'join') {
            addMessage(`${data.sender} has joined the chat`, false, null, 'join');
        }
    } catch (error) {
        console.error('Error handling message:', error);
        addMessage(event.data, false);
    }
};

input.addEventListener('keypress', (e) => {
    if (e.key === 'Enter' && input.value.trim()) {
        const messageToSend = JSON.stringify({
            type: 'message',
            content: input.value
        });
        ws.send(messageToSend);
        input.value = '';
    }
});

function addMessage(content, isRight, extraContent = null, messageType = null) {
    const messageDiv = document.createElement('div');
    
    if (messageType === 'join') {
        messageDiv.className = 'message join';
    } else {
        messageDiv.className = 'message ' + (isRight ? 'right' : 'left');
    }

    if (typeof content === 'string') {
        const textNode = document.createTextNode(content);
        messageDiv.appendChild(textNode);
    }

    if (extraContent) {
        messageDiv.appendChild(extraContent);
    }

    messages.appendChild(messageDiv);
    messages.scrollTop = messages.scrollHeight;
}