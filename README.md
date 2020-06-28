# rumose
**Ru**sty **Mo**ck **Se**rver for basic CRUD APIs built as a single executable file

Based on [json-server](https://github.com/typicode/json-server), this project will let you have a full fake REST API with zero coding in less than 29 seconds.

Steps:

1- Download the single executable file from the [releases]() (Windows or Linux)

2- Create a JSON file with your dummy data [like this]()
```
{
  "posts": [
    { "id": 1, "title": "rumose", "body": "omg so good!", "author": "ludeed" },
    { 
      "id": 2, 
      "title": "rumose for life", 
      "body": "I dont have to go to sketchy mock server sites anymore", 
      "author": "ludeed" 
    }
  ],
  "comments": [
    { "id": 1, "body": "Agree ! So good", "postId": 1 }
  ],
  "profile": { "name": "stuff" }
}
```
3 - Run the executable and pass the file as argument
```
Linux   :  $ ./rumose routes.json
Windows :  PS> .\rumose.exe routes.json
```
4 - Go to [127.0.0.1:1337/comments](http://127.0.0.1:1337/comments) and you will see something like
```
{ "id": 1, "body": "Agree ! So good", "postId": 1 }
```


NOTE: Not fully complete, feel free to browse the issues and pick one up even if you are not fluent with RUST (I am not either)
