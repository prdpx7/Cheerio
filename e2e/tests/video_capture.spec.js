const { test, expect } = require('@playwright/test');
const path = require('path');
const fs = require('fs');
const { GAME_URL } = require('./helpers');

test.describe('Video capture for visual validation', () => {
    test('capture loading sequence', async ({ browser }) => {
        test.setTimeout(60000);
        const ctx = await browser.newContext({
            viewport: { width: 1280, height: 720 },
            recordVideo: { dir: '/tmp/cheerio-videos/', size: { width: 1280, height: 720 } },
        });
        const page = await ctx.newPage();
        await page.goto(GAME_URL, { timeout: 30000 });
        await page.waitForTimeout(12000);
        await ctx.close();
        const files = fs.readdirSync('/tmp/cheerio-videos/');
        expect(files.length).toBeGreaterThan(0);
        console.log('    Loading video saved:', files[0]);
    });

    test('capture gameplay and death screen', async ({ browser }) => {
        test.setTimeout(120000);
        const ctx = await browser.newContext({
            viewport: { width: 1280, height: 720 },
            recordVideo: { dir: '/tmp/cheerio-videos/', size: { width: 1280, height: 720 } },
        });
        const page = await ctx.newPage();
        await page.goto(GAME_URL, { timeout: 30000 });
        await page.waitForTimeout(10000);
        await page.keyboard.press('Space');
        await page.waitForTimeout(500);
        for (let i = 0; i < 10; i++) {
            await page.keyboard.press('Space');
            await page.waitForTimeout(400);
        }
        await page.waitForTimeout(25000);
        await page.waitForTimeout(5000);
        await ctx.close();
        const files = fs.readdirSync('/tmp/cheerio-videos/').sort();
        console.log('    Death screen video saved:', files[files.length - 1]);
        expect(files.length).toBeGreaterThan(0);
    });
});
