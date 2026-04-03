const { defineConfig } = require('@playwright/test');

const BASE_URL = process.env.BASE_URL || 'https://prdpx7.github.io/Cheerio/';

module.exports = defineConfig({
    testDir: './tests',
    timeout: 90000,
    retries: 1,
    workers: 2,
    use: {
        baseURL: BASE_URL,
        screenshot: 'only-on-failure',
        browserName: 'chromium',
        launchOptions: {
            args: ['--use-gl=swiftshader', '--autoplay-policy=no-user-gesture-required'],
        },
    },
    projects: [
        {
            name: 'desktop',
            use: { viewport: { width: 1280, height: 720 } },
        },
        {
            name: 'iphone',
            use: {
                viewport: { width: 844, height: 390 },
                isMobile: true,
                hasTouch: true,
                userAgent: 'Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X) AppleWebKit/605.1.15',
            },
        },
        {
            name: 'android',
            use: {
                viewport: { width: 915, height: 412 },
                isMobile: true,
                hasTouch: true,
                userAgent: 'Mozilla/5.0 (Linux; Android 13; SM-S908B) AppleWebKit/537.36',
            },
        },
    ],
    reporter: [['list']],
});
