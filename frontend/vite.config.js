import path from "path";

import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { VitePWA } from "vite-plugin-pwa";
import Icons from "unplugin-icons/vite";

export default defineConfig({
	plugins: [
		svelte(),
		VitePWA({
			registerType: "autoUpdate",
			workbox: {
				globPatterns: ["**/*.{js,css,html,ico}"],
			},
			includeAssets: [
				"*.json",
				"translations/*.json",
				"translations/ui/*.ftl",
				"sprites/*.jpg",
				"*.otf",
			],
		}),
		Icons({ compiler: "svelte" }),
		injectMetrikaPlugin("101631901"),
	],
	base: "/ow-tracker",
	build: {
		sourcemap: true,
		rollupOptions: {
			treeshake: true,
		},
	},
	resolve: {
		alias: {
			"@": path.resolve(__dirname, "./src"),
		},
	},
});

/**
 * @param {string} id
 * @returns {import("vite").Plugin}
 */
function injectMetrikaPlugin(id) {
	return {
		name: "inject-yandex-metrika",
		// not in dev mode
		apply: "build",
		transformIndexHtml(html, ctx) {
			return {
				html,
				tags: [
					{
						tag: "raw",
						children: gen_ya_metrica(id),
						injectTo: "head",
					},
				],
			};
		},
	};
}

function gen_ya_metrica(id) {
	return `<!-- Yandex.Metrika counter -->
<script type="text/javascript" >
   (function(m,e,t,r,i,k,a){m[i]=m[i]||function(){(m[i].a=m[i].a||[]).push(arguments)};
   m[i].l=1*new Date();
   for (var j = 0; j < document.scripts.length; j++) {if (document.scripts[j].src === r) { return; }}
   k=e.createElement(t),a=e.getElementsByTagName(t)[0],k.async=1,k.src=r,a.parentNode.insertBefore(k,a)})
   (window, document, "script", "https://mc.yandex.ru/metrika/tag.js", "ym");

   ym(${id}, "init", {
        clickmap:true,
        trackLinks:true,
        accurateTrackBounce:true
   });
</script>
<noscript><div><img src="https://mc.yandex.ru/watch/${id}" style="position:absolute; left:-9999px;" alt="" /></div></noscript>
<!-- /Yandex.Metrika counter -->`;
}
