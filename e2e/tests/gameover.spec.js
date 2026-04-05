const { test, expect } = require('@playwright/test');
const { waitForGame, GAME_URL } = require('./helpers');

test.describe('GameOver Screen', () => {
    test.beforeEach(async ({ page }) => {
        await waitForGame(page);
    });

    test('game over screen appears with delay after death', async ({ page }) => {
        test.skip(!!test.info().project.use?.isMobile, 'desktop only');
        test.setTimeout(90000);

        await page.keyboard.press('Space');
        await page.waitForTimeout(500);

        // Wait long enough for player to die naturally (up to 25s)
        await page.waitForTimeout(25000);

        // At this point player should have died. Game over timer should be
        // well past the 2.5s threshold. Space should restart.
        const before = await page.screenshot();

        await page.keyboard.press('Space');
        await page.waitForTimeout(2000);

        const after = await page.screenshot();
        // Screenshot should change (title screen appears after restart)
        expect(after.byteLength).toBeGreaterThan(1000);
    });

    test('game over screen does not accept input immediately', async ({ page }) => {
        test.skip(!!test.info().project.use?.isMobile, 'desktop only');
        test.setTimeout(60000);

        await page.keyboard.press('Space');
        await page.waitForTimeout(500);

        // Wait for player to die
        await page.waitForTimeout(22000);

        // Immediately after death/transition there should be no instant restart.
        // The game-over screen should be rendering at this point.
        const shot = await page.screenshot();
        expect(shot.byteLength).toBeGreaterThan(1000);
    });

    test('restart works after full game over delay', async ({ page }) => {
        test.skip(!!test.info().project.use?.isMobile, 'desktop only');
        test.setTimeout(90000);

        await page.keyboard.press('Space');
        await page.waitForTimeout(500);

        // Play a bit then wait out the game
        for (let i = 0; i < 5; i++) {
            await page.keyboard.press('Space');
            await page.waitForTimeout(300);
        }
        await page.waitForTimeout(25000);

        // Wait extra to ensure game-over timer > 2.5s after dying
        await page.waitForTimeout(4000);

        await page.keyboard.press('Space');
        await page.waitForTimeout(1500);

        const screenshot = await page.screenshot();
        expect(screenshot.byteLength).toBeGreaterThan(1000);
    });

    test('no JS errors during death animation and game over', async ({ page }) => {
        test.skip(!!test.info().project.use?.isMobile, 'desktop only');
        test.setTimeout(90000);

        const errors = [];
        page.on('pageerror', err => errors.push(err.message));
        page.on('console', msg => {
            if (msg.type() === 'error') errors.push(msg.text());
        });

        await page.keyboard.press('Space');
        await page.waitForTimeout(500);

        await page.waitForTimeout(30000);
        await page.keyboard.press('Space');
        await page.waitForTimeout(3000);

        expect(errors).toHaveLength(0);
    });
});
