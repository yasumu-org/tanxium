Tanxium.setRuntimeData({
    request: {
        url: 'https://example.com',
        method: 'GET',
        headers: {
            'User-Agent': 'Tanxium/1.0.0',
        },
    },
    response: {
        status: 200,
        headers: {
            'Content-Type': 'text/html',
        },
        body: '<h1>Hello, World!</h1>',
    },
});