const express = require('express');

function doWork() {
    return 'Hello World!';
}

const app = express();
const port = 8081;

app.get('/run', (req, res) => {
    res.send(doWork());
});

app.listen(port, () => {
    console.log(`Example app listening on port ${port}`);
});