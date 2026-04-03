const { test, expect } = require('@playwright/test');
const { GAME_URL } = require('./helpers');

test.describe('Loading', () => {
    test('page loads and canvas renders', async ({ page }) => {
        await page.goto(GAME_URL, { timeout: 30000 });
        await page.waitForTimeout(12000);

        const canvas = await page.evaluate(() => {
            const c = document.getElementById('glcanvas');
            return { exists: !!c, width: c?.width || 0, height: c?.height || 0 };
        });
        expect(canvas.exists).toBe(true);
        expect(canvas.width).toBeGreaterThan(0);
    });

    test('all audio assets return 200', async ({ page }) => {
        const audioResponses = [];
        page.on('response', res => {
            if (res.url().includes('/assets/audio/')) {
                audioResponses.push({ file: res.url().split('/').pop(), status: res.status() });
            }
        });

        await page.goto(GAME_URL, { timeout: 30000 });
        await page.waitForTimeout(15000);

        const expectedFiles = [
            'smb_jump-small.wav', 'smb_coin.wav', 'smb_stomp.wav',
            'smb_powerup.wav', 'smb_fireball.wav', 'smb_mariodie.wav',
            'smb_1-up.wav', 'smb_bump.wav', 'bgm_main.ogg',
        ];

        for (const file of expectedFiles) {
            const match = audioResponses.find(r => r.file === file);
            expect(match, `${file} should load`).toBeDefined();
            expect(match.status).toBe(200);
        }
    });

    test('WASM binary loads', async ({ page }) => {
        let wasmLoaded = false;
        page.on('response', res => {
            if (res.url().includes('.wasm') && res.status() === 200) wasmLoaded = true;
        });

        await page.goto(GAME_URL, { timeout: 30000 });
        await page.waitForTimeout(12000);
        expect(wasmLoaded).toBe(true);
    });
});
