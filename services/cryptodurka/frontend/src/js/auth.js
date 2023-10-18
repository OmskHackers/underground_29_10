document.addEventListener("DOMContentLoaded", function () {
    document.getElementById("register-form").addEventListener("submit", function (e) {
        e.preventDefault();
    
        var formData = new FormData(document.getElementById("register-form"));
        var username = formData.get("username");
        var password = formData.get("password");

        fetch("/api/auth/register", {
            method: "POST",
            body: JSON.stringify({ username, password }),
            headers: {
                "Content-Type": "application/json"
            }
        })
        .then(function(response) {
            return response.json();
        })
        .then(function(data) {
            var responseMessage = document.getElementById("response-register");
            responseMessage.textContent = data.message;
        })
        .catch(function(error) {
            console.error("Some error: " + error);
        });
    });

    document.getElementById("login-form").addEventListener("submit", function (e) {
        e.preventDefault();
    
        var formData = new FormData(document.getElementById("login-form"));
        var username = formData.get("username");
        var password = formData.get("password");

        fetch("/api/auth/login", {
            method: "POST",
            body: JSON.stringify({ username, password }),
            headers: {
                "Content-Type": "application/json"
            }
        })
        .then(function(response) {
            return response.json();
        })
        .then(function(data) {
            var responseMessage = document.getElementById("response-login");
            console.log(data.message)
            responseMessage.textContent = data.message;
        })
        .catch(function(error) {
            console.error("Some error: " + error);
        });
    });
});