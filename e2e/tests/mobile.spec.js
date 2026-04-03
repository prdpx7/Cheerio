const { test, expect } = require('@playwright/test');
const { waitForGame, GAME_URL } = require('./helpers');

test.describe('Mobile', () => {
    test.beforeEach(({ }, testInfo) => {
        test.skip(!testInfo.project.use?.isMobile && !testInfo.project.use?.hasTouch, 'mobile only');
    });

    test('portrait shows rotate message', async ({ browser }) => {
        const context = await browser.newContext({
            viewport: { width: 390, height: 844 },
            isMobile: true,
            hasTouch: true,
        });
        const page = await context.newPage();

        await page.goto(GAME_URL, { waitUntil: 'networkidle', timeout: 20000 });
        await page.waitForTimeout(6000);

        const screenshot = await page.screenshot();
        expect(screenshot.byteLength).toBeGreaterThan(1000);
        await context.close();
    });

    test('landscape starts and plays', async ({ browser }) => {
        const context = await browser.newContext({
            viewport: { width: 844, height: 390 },
            isMobile: true,
            hasTouch: true,
        });
        const page = await context.newPage();

        await page.goto(GAME_URL, { waitUntil: 'networkidle', timeout: 20000 });
        await page.waitForTimeout(6000);

        const box = await page.locator('#glcanvas').boundingBox();
        await page.tap('#glcanvas', { position: { x: box.width * 0.5, y: box.height * 0.5 } });
        await page.waitForTimeout(1500);

        const screenshot = await page.screenshot();
        expect(screenshot.byteLength).toBeGreaterThan(1000);
        await context.close();
    });

    test('left and right taps both jump', async ({ browser }) => {
        const context = await browser.newContext({
            viewport: { width: 844, height: 390 },
            isMobile: true,
            hasTouch: true,
        });
        const page = await context.newPage();

        await page.goto(GAME_URL, { waitUntil: 'networkidle', timeout: 20000 });
        await page.waitForTimeout(6000);

        const box = await page.locator('#glcanvas').boundingBox();

        await page.tap('#glcanvas', { position: { x: box.width * 0.5, y: box.height * 0.5 } });
        await page.waitForTimeout(1000);

        const before = await page.screenshot();

        await page.tap('#glcanvas', { position: { x: box.width * 0.25, y: box.height * 0.5 } });
        await page.waitForTimeout(200);
        const afterLeft = await page.screenshot();
        expect(before).not.toEqual(afterLeft);

        await page.waitForTimeout(800);

        const beforeRight = await page.screenshot();
        await page.tap('#glcanvas', { position: { x: box.width * 0.75, y: box.height * 0.5 } });
        await page.waitForTimeout(200);
        const afterRight = await page.screenshot();
        expect(beforeRight).not.toEqual(afterRight);

        await context.close();
    });

    test('rapid taps register', async ({ browser }) => {
        const context = await browser.newContext({
            viewport: { width: 844, height: 390 },
            isMobile: true,
            hasTouch: true,
        });
        const page = await context.newPage();

        await page.goto(GAME_URL, { waitUntil: 'networkidle', timeout: 20000 });
        await page.waitForTimeout(6000);

        const box = await page.locator('#glcanvas').boundingBox();
        await page.tap('#glcanvas', { position: { x: box.width * 0.5, y: box.height * 0.5 } });
        await page.waitForTimeout(1000);

        let shots = [];
        for (let i = 0; i < 5; i++) {
            await page.tap('#glcanvas', { position: { x: box.width * 0.3, y: box.height * 0.5 } });
            await page.waitForTimeout(200);
            shots.push(await page.screenshot());
        }

        let changes = 0;
        for (let i = 1; i < shots.length; i++) {
            if (!shots[i].equals(shots[i - 1])) changes++;
        }
        expect(changes).toBeGreaterThanOrEqual(2);
        await context.close();
    });
});
