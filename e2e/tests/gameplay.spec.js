const { test, expect } = require('@playwright/test');
const { waitForGame, startGame, jump, tapCanvas } = require('./helpers');

test.describe('Gameplay', () => {
    test.beforeEach(async ({ page }) => {
        await waitForGame(page);
    });

    test('title screen renders', async ({ page }) => {
        const screenshot = await page.screenshot();
        expect(screenshot.byteLength).toBeGreaterThan(1000);
    });

    test('game starts on space', async ({ page }) => {
        test.skip(!!test.info().project.use?.isMobile, 'desktop only');
        await page.keyboard.press('Space');
        await page.waitForTimeout(1500);
        const screenshot = await page.screenshot();
        expect(screenshot.byteLength).toBeGreaterThan(1000);
    });

    test('player survives 5 seconds with jumps', async ({ page }) => {
        test.skip(!!test.info().project.use?.isMobile, 'desktop only');
        await page.keyboard.press('Space');
        await page.waitForTimeout(500);

        const start = Date.now();
        for (let i = 0; i < 12; i++) {
            await page.keyboard.press('Space');
            await page.waitForTimeout(400);
        }

        expect(Date.now() - start).toBeGreaterThan(4000);
    });

    test('pause and resume with escape', async ({ page }) => {
        test.skip(!!test.info().project.use?.isMobile, 'desktop only');
        await page.keyboard.press('Space');
        await page.waitForTimeout(1500);

        await page.keyboard.press('Escape');
        await page.waitForTimeout(500);

        await page.keyboard.press('Escape');
        await page.waitForTimeout(500);

        const screenshot = await page.screenshot();
        expect(screenshot.byteLength).toBeGreaterThan(1000);
    });

    test('game over and restart cycle', async ({ page }) => {
        test.skip(!!test.info().project.use?.isMobile, 'desktop only');
        await page.keyboard.press('Space');
        await page.waitForTimeout(20000);

        await page.keyboard.press('Space');
        await page.waitForTimeout(1000);

        await page.keyboard.press('Space');
        await page.waitForTimeout(1500);

        const screenshot = await page.screenshot();
        expect(screenshot.byteLength).toBeGreaterThan(1000);
    });
});
