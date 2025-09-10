<!doctype html>
<html class="">
	<head>
		<meta charset="utf-8" />
		<meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=no" />
		<meta name="description" content="We’re on a journey to advance and democratize artificial intelligence through open source and open science." />
		<meta property="fb:app_id" content="1321688464574422" />
		<meta name="twitter:card" content="summary_large_image" />
		<meta name="twitter:site" content="@huggingface" />
		<meta name="twitter:image" content="https://cdn-thumbnails.huggingface.co/social-thumbnails/models/anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4.png" />
		<meta property="og:title" content="chat.py · anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4 at main" />
		<meta property="og:type" content="website" />
		<meta property="og:url" content="https://huggingface.co/anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4/blob/main/chat.py" />
		<meta property="og:image" content="https://cdn-thumbnails.huggingface.co/social-thumbnails/models/anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4.png" />

		<link rel="stylesheet" href="/front/build/kube-ab0c01c/style.css" />

		<link rel="preconnect" href="https://fonts.gstatic.com" />
		<link
			href="https://fonts.googleapis.com/css2?family=Source+Sans+Pro:ital,wght@0,200;0,300;0,400;0,600;0,700;1,200;1,300;1,400;1,600;1,700&display=swap"
			rel="stylesheet"
		/>
		<link
			href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;600;700&display=swap"
			rel="stylesheet"
		/>

		<link
			rel="preload"
			href="https://cdnjs.cloudflare.com/ajax/libs/KaTeX/0.12.0/katex.min.css"
			as="style"
			onload="this.onload=null;this.rel='stylesheet'"
		/>
		<noscript>
			<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/KaTeX/0.12.0/katex.min.css" />
		</noscript>

		<script>const guestTheme = document.cookie.match(/theme=(\w+)/)?.[1]; document.documentElement.classList.toggle('dark', guestTheme === 'dark' || ( (!guestTheme || guestTheme === 'system') && window.matchMedia('(prefers-color-scheme: dark)').matches));</script>
<link rel="canonical" href="https://huggingface.co/anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4/blob/main/chat.py">  <!-- HEAD_svelte-1oal594_START --><style>.blob-line-num::before {
			content: attr(data-line-num);
		}
	</style><!-- HEAD_svelte-1oal594_END -->

		<title>chat.py · anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4 at main</title>

		<script
			defer
			data-domain="huggingface.co"
			event-loggedIn="false"
			src="/js/script.pageview-props.js"
		></script>
		<script>
			window.plausible =
				window.plausible ||
				function () {
					(window.plausible.q = window.plausible.q || []).push(arguments);
				};
		</script>
		<script>
			window.hubConfig = {"features":{"signupDisabled":false},"sshGitUrl":"git@hf.co","moonHttpUrl":"https:\/\/huggingface.co","captchaApiKey":"bd5f2066-93dc-4bdd-a64b-a24646ca3859","captchaDisabledOnSignup":true,"datasetViewerPublicUrl":"https:\/\/datasets-server.huggingface.co","stripePublicKey":"pk_live_x2tdjFXBCvXo2FFmMybezpeM00J6gPCAAc","environment":"production","userAgent":"HuggingFace (production)","spacesIframeDomain":"hf.space","spacesApiUrl":"https:\/\/api.hf.space","docSearchKey":"ece5e02e57300e17d152c08056145326e90c4bff3dd07d7d1ae40cf1c8d39cb6","logoDev":{"apiUrl":"https:\/\/img.logo.dev\/","apiKey":"pk_UHS2HZOeRnaSOdDp7jbd5w"}};
		</script>
		<script type="text/javascript" src="https://de5282c3ca0c.edge.sdk.awswaf.com/de5282c3ca0c/526cf06acb0d/challenge.js" defer></script> 
	</head>
	<body class="flex flex-col min-h-dvh bg-white dark:bg-gray-950 text-black ViewerBlobPage">
		<div class="flex min-h-dvh flex-col"><div class="SVELTE_HYDRATER contents" data-target="SystemThemeMonitor" data-props="{&quot;isLoggedIn&quot;:false}"></div>

	<div class="SVELTE_HYDRATER contents" data-target="MainHeader" data-props="{&quot;classNames&quot;:&quot;&quot;,&quot;isWide&quot;:false,&quot;isZh&quot;:false,&quot;isPro&quot;:false}"><header class="border-b border-gray-100 "><div class="w-full px-4 container flex h-16 items-center"><div class="flex flex-1 items-center"><a class="mr-5 flex flex-none items-center lg:mr-6" href="/"><img alt="Hugging Face's logo" class="w-7 md:mr-2" src="/front/assets/huggingface_logo-noborder.svg">
				<span class="hidden whitespace-nowrap text-lg font-bold md:block">Hugging Face</span></a>
			<div class="relative flex-1 lg:max-w-sm mr-2 sm:mr-4 md:mr-3 xl:mr-6"><input autocomplete="off" class="w-full dark:bg-gray-950 pl-8 form-input-alt h-9 pr-3 focus:shadow-xl " name="" placeholder="Search models, datasets, users..."   spellcheck="false" type="text" value="">
	<svg class="absolute left-2.5 text-gray-400 top-1/2 transform -translate-y-1/2" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" focusable="false" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 32 32"><path d="M30 28.59L22.45 21A11 11 0 1 0 21 22.45L28.59 30zM5 14a9 9 0 1 1 9 9a9 9 0 0 1-9-9z" fill="currentColor"></path></svg>
	</div>
			<div class="flex flex-none items-center justify-center p-0.5 place-self-stretch lg:hidden"><button class="relative z-40 flex h-6 w-8 items-center justify-center" type="button"><svg width="1em" height="1em" viewBox="0 0 10 10" class="text-xl" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" focusable="false" role="img" preserveAspectRatio="xMidYMid meet" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M1.65039 2.9999C1.65039 2.8066 1.80709 2.6499 2.00039 2.6499H8.00039C8.19369 2.6499 8.35039 2.8066 8.35039 2.9999C8.35039 3.1932 8.19369 3.3499 8.00039 3.3499H2.00039C1.80709 3.3499 1.65039 3.1932 1.65039 2.9999ZM1.65039 4.9999C1.65039 4.8066 1.80709 4.6499 2.00039 4.6499H8.00039C8.19369 4.6499 8.35039 4.8066 8.35039 4.9999C8.35039 5.1932 8.19369 5.3499 8.00039 5.3499H2.00039C1.80709 5.3499 1.65039 5.1932 1.65039 4.9999ZM2.00039 6.6499C1.80709 6.6499 1.65039 6.8066 1.65039 6.9999C1.65039 7.1932 1.80709 7.3499 2.00039 7.3499H8.00039C8.19369 7.3499 8.35039 7.1932 8.35039 6.9999C8.35039 6.8066 8.19369 6.6499 8.00039 6.6499H2.00039Z"></path></svg>
		</button>

	</div></div>
		<nav aria-label="Main" class="ml-auto hidden lg:block"><ul class="flex items-center gap-x-1 2xl:gap-x-2"><li class="hover:text-indigo-700"><a class="group flex items-center px-2 py-0.5 dark:text-gray-300 dark:hover:text-gray-100" href="/models"><svg class="mr-1.5 text-gray-400 group-hover:text-indigo-500" style="" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" focusable="false" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24"><path class="uim-quaternary" d="M20.23 7.24L12 12L3.77 7.24a1.98 1.98 0 0 1 .7-.71L11 2.76c.62-.35 1.38-.35 2 0l6.53 3.77c.29.173.531.418.7.71z" opacity=".25" fill="currentColor"></path><path class="uim-tertiary" d="M12 12v9.5a2.09 2.09 0 0 1-.91-.21L4.5 17.48a2.003 2.003 0 0 1-1-1.73v-7.5a2.06 2.06 0 0 1 .27-1.01L12 12z" opacity=".5" fill="currentColor"></path><path class="uim-primary" d="M20.5 8.25v7.5a2.003 2.003 0 0 1-1 1.73l-6.62 3.82c-.275.13-.576.198-.88.2V12l8.23-4.76c.175.308.268.656.27 1.01z" fill="currentColor"></path></svg>
						Models</a>
				</li><li class="hover:text-red-700"><a class="group flex items-center px-2 py-0.5 dark:text-gray-300 dark:hover:text-gray-100" href="/datasets"><svg class="mr-1.5 text-gray-400 group-hover:text-red-500" style="" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" focusable="false" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 25 25"><ellipse cx="12.5" cy="5" fill="currentColor" fill-opacity="0.25" rx="7.5" ry="2"></ellipse><path d="M12.5 15C16.6421 15 20 14.1046 20 13V20C20 21.1046 16.6421 22 12.5 22C8.35786 22 5 21.1046 5 20V13C5 14.1046 8.35786 15 12.5 15Z" fill="currentColor" opacity="0.5"></path><path d="M12.5 7C16.6421 7 20 6.10457 20 5V11.5C20 12.6046 16.6421 13.5 12.5 13.5C8.35786 13.5 5 12.6046 5 11.5V5C5 6.10457 8.35786 7 12.5 7Z" fill="currentColor" opacity="0.5"></path><path d="M5.23628 12C5.08204 12.1598 5 12.8273 5 13C5 14.1046 8.35786 15 12.5 15C16.6421 15 20 14.1046 20 13C20 12.8273 19.918 12.1598 19.7637 12C18.9311 12.8626 15.9947 13.5 12.5 13.5C9.0053 13.5 6.06886 12.8626 5.23628 12Z" fill="currentColor"></path></svg>
						Datasets</a>
				</li><li class="hover:text-blue-700"><a class="group flex items-center px-2 py-0.5 dark:text-gray-300 dark:hover:text-gray-100" href="/spaces"><svg class="mr-1.5 text-gray-400 group-hover:text-blue-500" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" focusable="false" role="img" width="1em" height="1em" viewBox="0 0 25 25"><path opacity=".5" d="M6.016 14.674v4.31h4.31v-4.31h-4.31ZM14.674 14.674v4.31h4.31v-4.31h-4.31ZM6.016 6.016v4.31h4.31v-4.31h-4.31Z" fill="currentColor"></path><path opacity=".75" fill-rule="evenodd" clip-rule="evenodd" d="M3 4.914C3 3.857 3.857 3 4.914 3h6.514c.884 0 1.628.6 1.848 1.414a5.171 5.171 0 0 1 7.31 7.31c.815.22 1.414.964 1.414 1.848v6.514A1.914 1.914 0 0 1 20.086 22H4.914A1.914 1.914 0 0 1 3 20.086V4.914Zm3.016 1.102v4.31h4.31v-4.31h-4.31Zm0 12.968v-4.31h4.31v4.31h-4.31Zm8.658 0v-4.31h4.31v4.31h-4.31Zm0-10.813a2.155 2.155 0 1 1 4.31 0 2.155 2.155 0 0 1-4.31 0Z" fill="currentColor"></path><path opacity=".25" d="M16.829 6.016a2.155 2.155 0 1 0 0 4.31 2.155 2.155 0 0 0 0-4.31Z" fill="currentColor"></path></svg>
						Spaces</a>
				</li><li class="max-xl:hidden relative"><div class="relative ">
	<button class="group flex items-center px-2 py-0.5 dark:text-gray-300 hover:text-yellow-700 dark:hover:text-gray-100 " type="button">
		<svg class="mr-1.5 mr-1.5 text-gray-400 text-yellow-500! group-hover:text-yellow-500" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" focusable="false" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 32 32"><path d="M20.6081 3C21.7684 3 22.8053 3.49196 23.5284 4.38415C23.9756 4.93678 24.4428 5.82749 24.4808 7.16133C24.9674 7.01707 25.4353 6.93643 25.8725 6.93643C26.9833 6.93643 27.9865 7.37587 28.696 8.17411C29.6075 9.19872 30.0124 10.4579 29.8361 11.7177C29.7523 12.3177 29.5581 12.8555 29.2678 13.3534C29.8798 13.8646 30.3306 14.5763 30.5485 15.4322C30.719 16.1032 30.8939 17.5006 29.9808 18.9403C30.0389 19.0342 30.0934 19.1319 30.1442 19.2318C30.6932 20.3074 30.7283 21.5229 30.2439 22.6548C29.5093 24.3704 27.6841 25.7219 24.1397 27.1727C21.9347 28.0753 19.9174 28.6523 19.8994 28.6575C16.9842 29.4379 14.3477 29.8345 12.0653 29.8345C7.87017 29.8345 4.8668 28.508 3.13831 25.8921C0.356375 21.6797 0.754104 17.8269 4.35369 14.1131C6.34591 12.058 7.67023 9.02782 7.94613 8.36275C8.50224 6.39343 9.97271 4.20438 12.4172 4.20438H12.4179C12.6236 4.20438 12.8314 4.2214 13.0364 4.25468C14.107 4.42854 15.0428 5.06476 15.7115 6.02205C16.4331 5.09583 17.134 4.359 17.7682 3.94323C18.7242 3.31737 19.6794 3 20.6081 3ZM20.6081 5.95917C20.2427 5.95917 19.7963 6.1197 19.3039 6.44225C17.7754 7.44319 14.8258 12.6772 13.7458 14.7131C13.3839 15.3952 12.7655 15.6837 12.2086 15.6837C11.1036 15.6837 10.2408 14.5497 12.1076 13.1085C14.9146 10.9402 13.9299 7.39584 12.5898 7.1776C12.5311 7.16799 12.4731 7.16355 12.4172 7.16355C11.1989 7.16355 10.6615 9.33114 10.6615 9.33114C10.6615 9.33114 9.0863 13.4148 6.38031 16.206C3.67434 18.998 3.5346 21.2388 5.50675 24.2246C6.85185 26.2606 9.42666 26.8753 12.0653 26.8753C14.8021 26.8753 17.6077 26.2139 19.1799 25.793C19.2574 25.7723 28.8193 22.984 27.6081 20.6107C27.4046 20.212 27.0693 20.0522 26.6471 20.0522C24.9416 20.0522 21.8393 22.6726 20.5057 22.6726C20.2076 22.6726 19.9976 22.5416 19.9116 22.222C19.3433 20.1173 28.552 19.2325 27.7758 16.1839C27.639 15.6445 27.2677 15.4256 26.746 15.4263C24.4923 15.4263 19.4358 19.5181 18.3759 19.5181C18.2949 19.5181 18.2368 19.4937 18.2053 19.4419C17.6743 18.557 17.9653 17.9394 21.7082 15.6009C25.4511 13.2617 28.0783 11.8545 26.5841 10.1752C26.4121 9.98141 26.1684 9.8956 25.8725 9.8956C23.6001 9.89634 18.2311 14.9403 18.2311 14.9403C18.2311 14.9403 16.7821 16.496 15.9057 16.496C15.7043 16.496 15.533 16.4139 15.4169 16.2112C14.7956 15.1296 21.1879 10.1286 21.5484 8.06535C21.7928 6.66715 21.3771 5.95917 20.6081 5.95917Z" fill="#FF9D00"></path><path d="M5.50686 24.2246C3.53472 21.2387 3.67446 18.9979 6.38043 16.206C9.08641 13.4147 10.6615 9.33111 10.6615 9.33111C10.6615 9.33111 11.2499 6.95933 12.59 7.17757C13.93 7.39581 14.9139 10.9401 12.1069 13.1084C9.29997 15.276 12.6659 16.7489 13.7459 14.713C14.8258 12.6772 17.7747 7.44316 19.304 6.44221C20.8326 5.44128 21.9089 6.00204 21.5484 8.06532C21.188 10.1286 14.795 15.1295 15.4171 16.2118C16.0391 17.2934 18.2312 14.9402 18.2312 14.9402C18.2312 14.9402 25.0907 8.49588 26.5842 10.1752C28.0776 11.8545 25.4512 13.2616 21.7082 15.6008C17.9646 17.9393 17.6744 18.557 18.2054 19.4418C18.7372 20.3266 26.9998 13.1351 27.7759 16.1838C28.5513 19.2324 19.3434 20.1173 19.9117 22.2219C20.48 24.3274 26.3979 18.2382 27.6082 20.6107C28.8193 22.9839 19.2574 25.7722 19.18 25.7929C16.0914 26.62 8.24723 28.3726 5.50686 24.2246Z" fill="#FFD21E"></path></svg>
			Community
		</button>
	
	
	</div>
				</li><li class="hover:text-yellow-700"><a class="group flex items-center px-2 py-0.5 dark:text-gray-300 dark:hover:text-gray-100" href="/docs"><svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" role="img" class="mr-1.5 text-gray-400 group-hover:text-yellow-500" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 16 16"><path d="m2.28 3.7-.3.16a.67.67 0 0 0-.34.58v8.73l.01.04.02.07.01.04.03.06.02.04.02.03.04.06.05.05.04.04.06.04.06.04.08.04.08.02h.05l.07.02h.11l.04-.01.07-.02.03-.01.07-.03.22-.12a5.33 5.33 0 0 1 5.15.1.67.67 0 0 0 .66 0 5.33 5.33 0 0 1 5.33 0 .67.67 0 0 0 1-.58V4.36a.67.67 0 0 0-.34-.5l-.3-.17v7.78a.63.63 0 0 1-.87.59 4.9 4.9 0 0 0-4.35.35l-.65.39a.29.29 0 0 1-.15.04.29.29 0 0 1-.16-.04l-.65-.4a4.9 4.9 0 0 0-4.34-.34.63.63 0 0 1-.87-.59V3.7Z" fill="currentColor" class="dark:opacity-40"></path><path fill-rule="evenodd" clip-rule="evenodd" d="M8 3.1a5.99 5.99 0 0 0-5.3-.43.66.66 0 0 0-.42.62v8.18c0 .45.46.76.87.59a4.9 4.9 0 0 1 4.34.35l.65.39c.05.03.1.04.16.04.05 0 .1-.01.15-.04l.65-.4a4.9 4.9 0 0 1 4.35-.34.63.63 0 0 0 .86-.59V3.3a.67.67 0 0 0-.41-.62 5.99 5.99 0 0 0-5.3.43l-.3.17L8 3.1Zm.73 1.87a.43.43 0 1 0-.86 0v5.48a.43.43 0 0 0 .86 0V4.97Z" fill="currentColor" class="opacity-40 dark:opacity-100"></path><path d="M8.73 4.97a.43.43 0 1 0-.86 0v5.48a.43.43 0 1 0 .86 0V4.96Z" fill="currentColor" class="dark:opacity-40"></path></svg>
						Docs</a>
				</li><li class="hover:text-black dark:hover:text-white max-2xl:hidden"><a class="group flex items-center px-2 py-0.5 dark:text-gray-300 dark:hover:text-gray-100" href="/enterprise"><svg class="mr-1.5 text-gray-400 group-hover:text-black dark:group-hover:text-white" xmlns="http://www.w3.org/2000/svg" fill="none" aria-hidden="true" focusable="false" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 12 12"><path fill-rule="evenodd" clip-rule="evenodd" d="M4.9 1.35a3.16 3.16 0 0 0-2.8 2.07L.37 8.58C0 9.71.7 10.65 1.86 10.65H7.3a3.2 3.2 0 0 0 2.84-2.07l1.67-5.16c.36-1.13-.3-2.07-1.46-2.07H4.91Zm.4 2.07L3.57 8.47h3.57l.36-1.12H5.4l.28-.91h1.75l.4-1.1H6.07l.3-.83h2l.36-1.1H5.27h.04Z" fill="currentColor"></path></svg>
						Enterprise</a>
				</li>

		<li><a class="group flex items-center px-2 py-0.5 dark:text-gray-300 dark:hover:text-gray-100" href="/pricing">Pricing
			</a></li>

		<li><div class="relative group">
	<button class="px-2 py-0.5 hover:text-gray-500 dark:hover:text-gray-600 flex items-center " type="button">
		<svg class=" text-gray-500 w-5 group-hover:text-gray-400 dark:text-gray-300 dark:group-hover:text-gray-100" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" focusable="false" role="img" width="1em" height="1em" viewBox="0 0 32 18" preserveAspectRatio="xMidYMid meet"><path fill-rule="evenodd" clip-rule="evenodd" d="M14.4504 3.30221C14.4504 2.836 14.8284 2.45807 15.2946 2.45807H28.4933C28.9595 2.45807 29.3374 2.836 29.3374 3.30221C29.3374 3.76842 28.9595 4.14635 28.4933 4.14635H15.2946C14.8284 4.14635 14.4504 3.76842 14.4504 3.30221Z" fill="currentColor"></path><path fill-rule="evenodd" clip-rule="evenodd" d="M14.4504 9.00002C14.4504 8.53382 14.8284 8.15588 15.2946 8.15588H28.4933C28.9595 8.15588 29.3374 8.53382 29.3374 9.00002C29.3374 9.46623 28.9595 9.84417 28.4933 9.84417H15.2946C14.8284 9.84417 14.4504 9.46623 14.4504 9.00002Z" fill="currentColor"></path><path fill-rule="evenodd" clip-rule="evenodd" d="M14.4504 14.6978C14.4504 14.2316 14.8284 13.8537 15.2946 13.8537H28.4933C28.9595 13.8537 29.3374 14.2316 29.3374 14.6978C29.3374 15.164 28.9595 15.542 28.4933 15.542H15.2946C14.8284 15.542 14.4504 15.164 14.4504 14.6978Z" fill="currentColor"></path><path fill-rule="evenodd" clip-rule="evenodd" d="M1.94549 6.87377C2.27514 6.54411 2.80962 6.54411 3.13928 6.87377L6.23458 9.96907L9.32988 6.87377C9.65954 6.54411 10.194 6.54411 10.5237 6.87377C10.8533 7.20343 10.8533 7.73791 10.5237 8.06756L6.23458 12.3567L1.94549 8.06756C1.61583 7.73791 1.61583 7.20343 1.94549 6.87377Z" fill="currentColor"></path></svg>
			
		</button>
	
	
	</div></li>
		<li><hr class="h-5 w-0.5 border-none bg-gray-100 dark:bg-gray-800"></li>
		<li><a class="block cursor-pointer whitespace-nowrap px-2 py-0.5 hover:text-gray-500 dark:text-gray-300 dark:hover:text-gray-100" href="/login">Log In
				</a></li>
			<li><a class="whitespace-nowrap rounded-full border border-transparent bg-gray-900 px-3 py-1 leading-none text-white hover:border-black hover:bg-white hover:text-black" href="/join">Sign Up
					</a></li></ul></nav></div></header></div>
	
	
	
	<div class="SVELTE_HYDRATER contents" data-target="SSOBanner" data-props="{}"></div>
	



	<main class="flex flex-1 flex-col">
	<div class="SVELTE_HYDRATER contents" data-target="ModelHeader" data-props="{&quot;activeTab&quot;:&quot;files&quot;,&quot;author&quot;:{&quot;_id&quot;:&quot;679d9680de0c0f8370cabcf3&quot;,&quot;avatarUrl&quot;:&quot;https://cdn-avatars.huggingface.co/v1/production/uploads/679d9680de0c0f8370cabcf3/numfzR_Lto_Hkvk-Pj8l8.png&quot;,&quot;fullname&quot;:&quot;ANEMLL: Open Source project for TPU models&quot;,&quot;name&quot;:&quot;anemll&quot;,&quot;type&quot;:&quot;user&quot;,&quot;isPro&quot;:false,&quot;isHf&quot;:false,&quot;isHfAdmin&quot;:false,&quot;isMod&quot;:false,&quot;followerCount&quot;:67},&quot;canReadRepoSettings&quot;:false,&quot;canWriteRepoContent&quot;:false,&quot;canDisable&quot;:false,&quot;model&quot;:{&quot;author&quot;:&quot;anemll&quot;,&quot;cardData&quot;:{&quot;license&quot;:&quot;mit&quot;,&quot;tags&quot;:[&quot;coreml&quot;,&quot;ANE&quot;,&quot;LLaMA&quot;,&quot;Qwen&quot;,&quot;DeepSeek&quot;,&quot;Apple&quot;,&quot;Apple Neural Engine&quot;,&quot;DeepHermes&quot;]},&quot;cardExists&quot;:true,&quot;config&quot;:{&quot;model_type&quot;:&quot;llama&quot;,&quot;tokenizer_config&quot;:{&quot;bos_token&quot;:null,&quot;chat_template&quot;:&quot;{%- if tools %}\n    {{- '<|im_start|>system\\n' }}\n    {%- if messages[0].role == 'system' %}\n        {{- messages[0].content + '\\n\\n' }}\n    {%- endif %}\n    {{- \&quot;# Tools\\n\\nYou may call one or more functions to assist with the user query.\\n\\nYou are provided with function signatures within <tools></tools> XML tags:\\n<tools>\&quot; }}\n    {%- for tool in tools %}\n        {{- \&quot;\\n\&quot; }}\n        {{- tool | tojson }}\n    {%- endfor %}\n    {{- \&quot;\\n</tools>\\n\\nFor each function call, return a json object with function name and arguments within <tool_call></tool_call> XML tags:\\n<tool_call>\\n{\\\&quot;name\\\&quot;: <function-name>, \\\&quot;arguments\\\&quot;: <args-json-object>}\\n</tool_call><|im_end|>\\n\&quot; }}\n{%- else %}\n    {%- if messages[0].role == 'system' %}\n        {{- '<|im_start|>system\\n' + messages[0].content + '<|im_end|>\\n' }}\n    {%- endif %}\n{%- endif %}\n{%- set ns = namespace(multi_step_tool=true, last_query_index=messages|length - 1) %}\n{%- for message in messages[::-1] %}\n    {%- set index = (messages|length - 1) - loop.index0 %}\n    {%- if ns.multi_step_tool and message.role == \&quot;user\&quot; and message.content is string and not(message.content.startswith('<tool_response>') and message.content.endswith('</tool_response>')) %}\n        {%- set ns.multi_step_tool = false %}\n        {%- set ns.last_query_index = index %}\n    {%- endif %}\n{%- endfor %}\n{%- for message in messages %}\n    {%- if message.content is string %}\n        {%- set content = message.content %}\n    {%- else %}\n        {%- set content = '' %}\n    {%- endif %}\n    {%- if (message.role == \&quot;user\&quot;) or (message.role == \&quot;system\&quot; and not loop.first) %}\n        {{- '<|im_start|>' + message.role + '\\n' + content + '<|im_end|>' + '\\n' }}\n    {%- elif message.role == \&quot;assistant\&quot; %}\n        {%- set reasoning_content = '' %}\n        {%- if message.reasoning_content is string %}\n            {%- set reasoning_content = message.reasoning_content %}\n        {%- else %}\n            {%- if '</think>' in content %}\n                {%- set reasoning_content = content.split('</think>')[0].rstrip('\\n').split('<think>')[-1].lstrip('\\n') %}\n                {%- set content = content.split('</think>')[-1].lstrip('\\n') %}\n            {%- endif %}\n        {%- endif %}\n        {%- if loop.index0 > ns.last_query_index %}\n            {%- if loop.last or (not loop.last and reasoning_content) %}\n                {{- '<|im_start|>' + message.role + '\\n<think>\\n' + reasoning_content.strip('\\n') + '\\n</think>\\n\\n' + content.lstrip('\\n') }}\n            {%- else %}\n                {{- '<|im_start|>' + message.role + '\\n' + content }}\n            {%- endif %}\n        {%- else %}\n            {{- '<|im_start|>' + message.role + '\\n' + content }}\n        {%- endif %}\n        {%- if message.tool_calls %}\n            {%- for tool_call in message.tool_calls %}\n                {%- if (loop.first and content) or (not loop.first) %}\n                    {{- '\\n' }}\n                {%- endif %}\n                {%- if tool_call.function %}\n                    {%- set tool_call = tool_call.function %}\n                {%- endif %}\n                {{- '<tool_call>\\n{\&quot;name\&quot;: \&quot;' }}\n                {{- tool_call.name }}\n                {{- '\&quot;, \&quot;arguments\&quot;: ' }}\n                {%- if tool_call.arguments is string %}\n                    {{- tool_call.arguments }}\n                {%- else %}\n                    {{- tool_call.arguments | tojson }}\n                {%- endif %}\n                {{- '}\\n</tool_call>' }}\n            {%- endfor %}\n        {%- endif %}\n        {{- '<|im_end|>\\n' }}\n    {%- elif message.role == \&quot;tool\&quot; %}\n        {%- if loop.first or (messages[loop.index0 - 1].role != \&quot;tool\&quot;) %}\n            {{- '<|im_start|>user' }}\n        {%- endif %}\n        {{- '\\n<tool_response>\\n' }}\n        {{- content }}\n        {{- '\\n</tool_response>' }}\n        {%- if loop.last or (messages[loop.index0 + 1].role != \&quot;tool\&quot;) %}\n            {{- '<|im_end|>\\n' }}\n        {%- endif %}\n    {%- endif %}\n{%- endfor %}\n{%- if add_generation_prompt %}\n    {{- '<|im_start|>assistant\\n' }}\n    {%- if enable_thinking is defined and enable_thinking is false %}\n        {{- '<think>\\n\\n</think>\\n\\n' }}\n    {%- endif %}\n{%- endif %}&quot;,&quot;eos_token&quot;:&quot;<|im_end|>&quot;,&quot;pad_token&quot;:&quot;<|endoftext|>&quot;,&quot;unk_token&quot;:null}},&quot;createdAt&quot;:&quot;2025-07-07T20:15:38.000Z&quot;,&quot;discussionsDisabled&quot;:false,&quot;downloads&quot;:20,&quot;downloadsAllTime&quot;:36,&quot;id&quot;:&quot;anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4&quot;,&quot;isLikedByUser&quot;:false,&quot;availableInferenceProviders&quot;:[],&quot;inference&quot;:&quot;&quot;,&quot;lastModified&quot;:&quot;2025-07-07T21:00:33.000Z&quot;,&quot;likes&quot;:0,&quot;librariesOther&quot;:[],&quot;trackDownloads&quot;:true,&quot;model-index&quot;:null,&quot;private&quot;:false,&quot;repoType&quot;:&quot;model&quot;,&quot;gated&quot;:false,&quot;pwcLink&quot;:{&quot;error&quot;:&quot;Unknown error, can't generate link to Papers With Code.&quot;},&quot;tags&quot;:[&quot;llama&quot;,&quot;coreml&quot;,&quot;ANE&quot;,&quot;LLaMA&quot;,&quot;Qwen&quot;,&quot;DeepSeek&quot;,&quot;Apple&quot;,&quot;Apple Neural Engine&quot;,&quot;DeepHermes&quot;,&quot;license:mit&quot;,&quot;region:us&quot;],&quot;tag_objs&quot;:[{&quot;id&quot;:&quot;coreml&quot;,&quot;label&quot;:&quot;Core ML&quot;,&quot;type&quot;:&quot;library&quot;},{&quot;id&quot;:&quot;llama&quot;,&quot;label&quot;:&quot;llama&quot;,&quot;type&quot;:&quot;other&quot;},{&quot;id&quot;:&quot;ANE&quot;,&quot;label&quot;:&quot;ANE&quot;,&quot;type&quot;:&quot;other&quot;},{&quot;id&quot;:&quot;LLaMA&quot;,&quot;label&quot;:&quot;LLaMA&quot;,&quot;type&quot;:&quot;other&quot;},{&quot;id&quot;:&quot;Qwen&quot;,&quot;label&quot;:&quot;Qwen&quot;,&quot;type&quot;:&quot;other&quot;},{&quot;id&quot;:&quot;DeepSeek&quot;,&quot;label&quot;:&quot;DeepSeek&quot;,&quot;type&quot;:&quot;other&quot;},{&quot;id&quot;:&quot;Apple&quot;,&quot;label&quot;:&quot;Apple&quot;,&quot;type&quot;:&quot;other&quot;},{&quot;id&quot;:&quot;Apple Neural Engine&quot;,&quot;label&quot;:&quot;Apple Neural Engine&quot;,&quot;type&quot;:&quot;other&quot;},{&quot;id&quot;:&quot;DeepHermes&quot;,&quot;label&quot;:&quot;DeepHermes&quot;,&quot;type&quot;:&quot;other&quot;},{&quot;id&quot;:&quot;license:mit&quot;,&quot;label&quot;:&quot;mit&quot;,&quot;type&quot;:&quot;license&quot;},{&quot;type&quot;:&quot;region&quot;,&quot;label&quot;:&quot;🇺🇸 Region: US&quot;,&quot;id&quot;:&quot;region:us&quot;}],&quot;hasBlockedOids&quot;:false,&quot;region&quot;:&quot;us&quot;,&quot;isQuantized&quot;:false,&quot;xetEnabled&quot;:true},&quot;discussionsStats&quot;:{&quot;closed&quot;:0,&quot;open&quot;:0,&quot;total&quot;:0},&quot;query&quot;:{},&quot;inferenceContextData&quot;:{&quot;billableEntities&quot;:[],&quot;entityName2Providers&quot;:{}}}"><header class="bg-linear-to-t border-b border-gray-100 pt-6 sm:pt-9 from-purple-500/8 dark:from-purple-500/20 to-white to-70%  dark:to-gray-950"><div class="container relative "><h1 class="flex flex-wrap items-center max-md:leading-tight mb-3 text-lg max-sm:gap-y-1.5 md:text-xl">
			<div class="group flex flex-none items-center"><div class="relative mr-1 flex items-center">

			

<span class="inline-block "><span class="contents"><a href="/anemll" class="text-gray-400 hover:text-blue-600"><img alt="" class="size-3.5 rounded-full  flex-none" src="https://cdn-avatars.huggingface.co/v1/production/uploads/679d9680de0c0f8370cabcf3/numfzR_Lto_Hkvk-Pj8l8.png" crossorigin="anonymous"></a></span>
	</span></div>
		

<span class="inline-block "><span class="contents"><a href="/anemll" class="text-gray-400 hover:text-blue-600">anemll</a></span>
	</span>
		<div class="mx-0.5 text-gray-300">/</div></div>

<div class="max-w-full "><a class="break-words font-mono font-semibold hover:text-blue-600 " href="/anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4">anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4</a>
	<button class="text-sm mr-4 focus:outline-hidden inline-flex cursor-pointer items-center text-sm  mx-0.5   text-gray-600 " title="Copy model name to clipboard" type="button"><svg class="" xmlns="http://www.w3.org/2000/svg" aria-hidden="true" fill="currentColor" focusable="false" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 32 32"><path d="M28,10V28H10V10H28m0-2H10a2,2,0,0,0-2,2V28a2,2,0,0,0,2,2H28a2,2,0,0,0,2-2V10a2,2,0,0,0-2-2Z" transform="translate(0)"></path><path d="M4,18H2V4A2,2,0,0,1,4,2H18V4H4Z" transform="translate(0)"></path><rect fill="none" width="32" height="32"></rect></svg>
		</button></div>
			<div class="inline-flex items-center overflow-hidden whitespace-nowrap rounded-md border bg-white text-sm leading-none text-gray-500  mr-2"><button class="relative flex items-center overflow-hidden from-red-50 to-transparent dark:from-red-900 px-1.5 py-1 hover:bg-linear-to-t focus:outline-hidden"  title="Like"><svg class="left-1.5 absolute" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" focusable="false" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 32 32" fill="currentColor"><path d="M22.45,6a5.47,5.47,0,0,1,3.91,1.64,5.7,5.7,0,0,1,0,8L16,26.13,5.64,15.64a5.7,5.7,0,0,1,0-8,5.48,5.48,0,0,1,7.82,0L16,10.24l2.53-2.58A5.44,5.44,0,0,1,22.45,6m0-2a7.47,7.47,0,0,0-5.34,2.24L16,7.36,14.89,6.24a7.49,7.49,0,0,0-10.68,0,7.72,7.72,0,0,0,0,10.82L16,29,27.79,17.06a7.72,7.72,0,0,0,0-10.82A7.49,7.49,0,0,0,22.45,4Z"></path></svg>

		
		<span class="ml-4 pl-0.5 ">like</span></button>
	<button class="focus:outline-hidden flex items-center border-l px-1.5 py-1 text-gray-400 hover:bg-gray-50 focus:bg-gray-100 dark:hover:bg-gray-900 dark:focus:bg-gray-800" title="See users who liked this repository">0</button></div>


			
			
	</h1>
		<div class="mb-3 flex flex-wrap md:mb-4"><a class="mb-1 mr-1 md:mb-1.5 md:mr-1.5 rounded-lg" href="/models?library=coreml"><div class="tag   tag-white "><svg class="text-black inline-block text-sm" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" focusable="false" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 734 734"><path d="M476.841 682.161L685.922 532.632C707.042 517.527 718.051 495.675 718.941 473.535V259.914H16.1792V473.535C17.0788 495.915 28.3135 518.001 49.8743 533.109L263.968 683.123C326.853 727.186 414.437 726.791 476.841 682.161Z" fill="#02C0A8"></path><path d="M476.841 581.103L685.922 431.573C707.042 416.468 718.051 394.616 718.941 372.476V268.5H16.1792V372.476C17.0788 394.856 28.3135 416.942 49.8743 432.05L263.968 582.065C326.853 626.128 414.437 625.732 476.841 581.103Z" fill="url(#paint0_linear_333_277)"></path><path d="M49.8739 326.114C4.93902 294.628 4.85527 232.827 49.7047 201.241L263.624 50.5795C326.643 6.19495 414.642 6.59392 477.176 51.5461L686.09 201.722C730.039 233.314 729.957 294.144 685.922 325.637L476.841 475.166C414.437 519.796 326.853 520.192 263.968 476.128L49.8739 326.114Z" fill="url(#paint1_linear_333_277)"></path><path d="M527.914 280.62L349.792 152.852L381.876 129.116L534.552 238.199L616.975 178.607L643.527 198.303L527.914 280.62Z" fill="white"></path><path d="M353.111 407.378L320.474 433.134L140.139 301.326L178.861 274.055L371.366 330.111L288.943 194.263L328.218 166.992L508 295.265L478.128 317.486L353.111 224.059L425.024 354.352L404.556 371.522L222.562 317.486L353.111 407.378Z" fill="white"></path><defs><linearGradient id="paint0_linear_333_277" x1="367.56" y1="325.566" x2="367.56" y2="748.767" gradientUnits="userSpaceOnUse"><stop stop-color="#CBF3FF"></stop><stop offset="1" stop-color="#CBF3FF" stop-opacity="0"></stop></linearGradient><linearGradient id="paint1_linear_333_277" x1="156.734" y1="113.461" x2="488.495" y2="473.957" gradientUnits="userSpaceOnUse"><stop stop-color="#02C5A8"></stop><stop offset="1" stop-color="#0186A7"></stop></linearGradient></defs></svg>

	

	<span>Core ML</span>
	

	</div></a><a class="mb-1 mr-1 md:mb-1.5 md:mr-1.5 rounded-lg" href="/models?other=llama"><div class="tag   tag-white ">

	

	<span>llama</span>
	

	</div></a><a class="mb-1 mr-1 md:mb-1.5 md:mr-1.5 rounded-lg" href="/models?other=ANE"><div class="tag   tag-white ">

	

	<span>ANE</span>
	

	</div></a><a class="mb-1 mr-1 md:mb-1.5 md:mr-1.5 rounded-lg" href="/models?other=LLaMA"><div class="tag   tag-white ">

	

	<span>LLaMA</span>
	

	</div></a><a class="mb-1 mr-1 md:mb-1.5 md:mr-1.5 rounded-lg" href="/models?other=Qwen"><div class="tag   tag-white ">

	

	<span>Qwen</span>
	

	</div></a><a class="mb-1 mr-1 md:mb-1.5 md:mr-1.5 rounded-lg" href="/models?other=DeepSeek"><div class="tag   tag-white ">

	

	<span>DeepSeek</span>
	

	</div></a><a class="mb-1 mr-1 md:mb-1.5 md:mr-1.5 rounded-lg" href="/models?other=Apple"><div class="tag   tag-white ">

	

	<span>Apple</span>
	

	</div></a><a class="mb-1 mr-1 md:mb-1.5 md:mr-1.5 rounded-lg" href="/models?other=Apple+Neural+Engine"><div class="tag   tag-white ">

	

	<span>Apple Neural Engine</span>
	

	</div></a><a class="mb-1 mr-1 md:mb-1.5 md:mr-1.5 rounded-lg" href="/models?other=DeepHermes"><div class="tag   tag-white ">

	

	<span>DeepHermes</span>
	

	</div></a><div class="relative inline-block ">
	<button class="group mr-1 mb-1 md:mr-1.5 md:mb-1.5  rounded-full rounded-br-none " type="button">
		<div slot="button"><div class="tag rounded-full  tag-white relative rounded-br-none pr-2.5">
		<svg class="text-xs text-gray-900" width="1em" height="1em" viewBox="0 0 10 10" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M1.46009 5.0945V6.88125C1.46009 7.25201 1.75937 7.55129 2.13012 7.55129C2.50087 7.55129 2.80016 7.25201 2.80016 6.88125V5.0945C2.80016 4.72375 2.50087 4.42446 2.13012 4.42446C1.75937 4.42446 1.46009 4.72375 1.46009 5.0945ZM4.14022 5.0945V6.88125C4.14022 7.25201 4.4395 7.55129 4.81026 7.55129C5.18101 7.55129 5.48029 7.25201 5.48029 6.88125V5.0945C5.48029 4.72375 5.18101 4.42446 4.81026 4.42446C4.4395 4.42446 4.14022 4.72375 4.14022 5.0945ZM1.23674 9.78473H8.38377C8.75452 9.78473 9.0538 9.48545 9.0538 9.1147C9.0538 8.74395 8.75452 8.44466 8.38377 8.44466H1.23674C0.865993 8.44466 0.566711 8.74395 0.566711 9.1147C0.566711 9.48545 0.865993 9.78473 1.23674 9.78473ZM6.82036 5.0945V6.88125C6.82036 7.25201 7.11964 7.55129 7.49039 7.55129C7.86114 7.55129 8.16042 7.25201 8.16042 6.88125V5.0945C8.16042 4.72375 7.86114 4.42446 7.49039 4.42446C7.11964 4.42446 6.82036 4.72375 6.82036 5.0945ZM4.39484 0.623142L0.865993 2.48137C0.682851 2.57517 0.566711 2.76725 0.566711 2.97273C0.566711 3.28094 0.816857 3.53109 1.12507 3.53109H8.49991C8.80365 3.53109 9.0538 3.28094 9.0538 2.97273C9.0538 2.76725 8.93766 2.57517 8.75452 2.48137L5.22568 0.623142C4.9666 0.484669 4.65391 0.484669 4.39484 0.623142V0.623142Z" fill="currentColor"></path></svg>

	<span class="-mr-1 text-gray-400">License:</span>

	<span>mit</span>
	

	<div class="border-br-gray-200 absolute bottom-0.5 right-0.5 h-1 w-1 border-[3px] border-l-transparent border-t-transparent border-b-gray-200 border-r-gray-200 group-hover:border-b-gray-400 group-hover:border-r-gray-400 dark:border-b-gray-700 dark:border-r-gray-700 group-hover:dark:border-b-gray-400 group-hover:dark:border-r-gray-400"></div></div></div>
		</button>
	
	
	</div></div>

		<div class="flex flex-col-reverse lg:flex-row lg:items-center lg:justify-between"><div class="-mb-px flex h-12 items-center overflow-x-auto overflow-y-hidden ">
	<a class="tab-alternate" href="/anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4"><svg class="mr-1.5 text-gray-400 flex-none" style="" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" focusable="false" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24"><path class="uim-quaternary" d="M20.23 7.24L12 12L3.77 7.24a1.98 1.98 0 0 1 .7-.71L11 2.76c.62-.35 1.38-.35 2 0l6.53 3.77c.29.173.531.418.7.71z" opacity=".25" fill="currentColor"></path><path class="uim-tertiary" d="M12 12v9.5a2.09 2.09 0 0 1-.91-.21L4.5 17.48a2.003 2.003 0 0 1-1-1.73v-7.5a2.06 2.06 0 0 1 .27-1.01L12 12z" opacity=".5" fill="currentColor"></path><path class="uim-primary" d="M20.5 8.25v7.5a2.003 2.003 0 0 1-1 1.73l-6.62 3.82c-.275.13-.576.198-.88.2V12l8.23-4.76c.175.308.268.656.27 1.01z" fill="currentColor"></path></svg>
	Model card
	

	
		</a><a class="tab-alternate active" href="/anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4/tree/main"><svg class="mr-1.5 text-gray-400 flex-none" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" focusable="false" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24"><path class="uim-tertiary" d="M21 19h-8a1 1 0 0 1 0-2h8a1 1 0 0 1 0 2zm0-4h-8a1 1 0 0 1 0-2h8a1 1 0 0 1 0 2zm0-8h-8a1 1 0 0 1 0-2h8a1 1 0 0 1 0 2zm0 4h-8a1 1 0 0 1 0-2h8a1 1 0 0 1 0 2z" opacity=".5" fill="currentColor"></path><path class="uim-primary" d="M9 19a1 1 0 0 1-1-1V6a1 1 0 0 1 2 0v12a1 1 0 0 1-1 1zm-6-4.333a1 1 0 0 1-.64-1.769L3.438 12l-1.078-.898a1 1 0 0 1 1.28-1.538l2 1.667a1 1 0 0 1 0 1.538l-2 1.667a.999.999 0 0 1-.64.231z" fill="currentColor"></path></svg>
	<span class="xl:hidden">Files</span>
		<span class="hidden xl:inline">Files and versions</span>
	

	

<span class="inline-block "><span class="contents"><div slot="anchor" class="shadow-purple-500/10 ml-2 inline-flex -translate-y-px items-center gap-0.5 rounded-md border bg-white px-1 py-0.5 align-middle text-xs font-semibold leading-none text-gray-800 shadow-sm dark:border-gray-700 dark:bg-gradient-to-b dark:from-gray-925 dark:to-gray-925 dark:text-gray-300"><svg class="size-3 " xmlns="http://www.w3.org/2000/svg" aria-hidden="true" fill="currentColor" focusable="false" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 12 12"><path fill-rule="evenodd" clip-rule="evenodd" d="M6.14 3.64 5.1 4.92 2.98 2.28h2.06l1.1 1.36Zm0 4.72-1.1 1.36H2.98l2.13-2.64 1.03 1.28Zm4.9 1.36L8.03 6l3-3.72H8.96L5.97 6l3 3.72h2.06Z" fill="#7875FF"></path><path d="M4.24 6 2.6 8.03.97 6 2.6 3.97 4.24 6Z" fill="#FF7F41" opacity="1"></path></svg>
						<span>xet</span>
					</div></span>
	</span>
		</a><a class="tab-alternate" href="/anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4/discussions"><svg class="mr-1.5 text-gray-400 flex-none" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" focusable="false" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 32 32"><path d="M20.6081 3C21.7684 3 22.8053 3.49196 23.5284 4.38415C23.9756 4.93678 24.4428 5.82749 24.4808 7.16133C24.9674 7.01707 25.4353 6.93643 25.8725 6.93643C26.9833 6.93643 27.9865 7.37587 28.696 8.17411C29.6075 9.19872 30.0124 10.4579 29.8361 11.7177C29.7523 12.3177 29.5581 12.8555 29.2678 13.3534C29.8798 13.8646 30.3306 14.5763 30.5485 15.4322C30.719 16.1032 30.8939 17.5006 29.9808 18.9403C30.0389 19.0342 30.0934 19.1319 30.1442 19.2318C30.6932 20.3074 30.7283 21.5229 30.2439 22.6548C29.5093 24.3704 27.6841 25.7219 24.1397 27.1727C21.9347 28.0753 19.9174 28.6523 19.8994 28.6575C16.9842 29.4379 14.3477 29.8345 12.0653 29.8345C7.87017 29.8345 4.8668 28.508 3.13831 25.8921C0.356375 21.6797 0.754104 17.8269 4.35369 14.1131C6.34591 12.058 7.67023 9.02782 7.94613 8.36275C8.50224 6.39343 9.97271 4.20438 12.4172 4.20438H12.4179C12.6236 4.20438 12.8314 4.2214 13.0364 4.25468C14.107 4.42854 15.0428 5.06476 15.7115 6.02205C16.4331 5.09583 17.134 4.359 17.7682 3.94323C18.7242 3.31737 19.6794 3 20.6081 3ZM20.6081 5.95917C20.2427 5.95917 19.7963 6.1197 19.3039 6.44225C17.7754 7.44319 14.8258 12.6772 13.7458 14.7131C13.3839 15.3952 12.7655 15.6837 12.2086 15.6837C11.1036 15.6837 10.2408 14.5497 12.1076 13.1085C14.9146 10.9402 13.9299 7.39584 12.5898 7.1776C12.5311 7.16799 12.4731 7.16355 12.4172 7.16355C11.1989 7.16355 10.6615 9.33114 10.6615 9.33114C10.6615 9.33114 9.0863 13.4148 6.38031 16.206C3.67434 18.998 3.5346 21.2388 5.50675 24.2246C6.85185 26.2606 9.42666 26.8753 12.0653 26.8753C14.8021 26.8753 17.6077 26.2139 19.1799 25.793C19.2574 25.7723 28.8193 22.984 27.6081 20.6107C27.4046 20.212 27.0693 20.0522 26.6471 20.0522C24.9416 20.0522 21.8393 22.6726 20.5057 22.6726C20.2076 22.6726 19.9976 22.5416 19.9116 22.222C19.3433 20.1173 28.552 19.2325 27.7758 16.1839C27.639 15.6445 27.2677 15.4256 26.746 15.4263C24.4923 15.4263 19.4358 19.5181 18.3759 19.5181C18.2949 19.5181 18.2368 19.4937 18.2053 19.4419C17.6743 18.557 17.9653 17.9394 21.7082 15.6009C25.4511 13.2617 28.0783 11.8545 26.5841 10.1752C26.4121 9.98141 26.1684 9.8956 25.8725 9.8956C23.6001 9.89634 18.2311 14.9403 18.2311 14.9403C18.2311 14.9403 16.7821 16.496 15.9057 16.496C15.7043 16.496 15.533 16.4139 15.4169 16.2112C14.7956 15.1296 21.1879 10.1286 21.5484 8.06535C21.7928 6.66715 21.3771 5.95917 20.6081 5.95917Z" fill="#FF9D00"></path><path d="M5.50686 24.2246C3.53472 21.2387 3.67446 18.9979 6.38043 16.206C9.08641 13.4147 10.6615 9.33111 10.6615 9.33111C10.6615 9.33111 11.2499 6.95933 12.59 7.17757C13.93 7.39581 14.9139 10.9401 12.1069 13.1084C9.29997 15.276 12.6659 16.7489 13.7459 14.713C14.8258 12.6772 17.7747 7.44316 19.304 6.44221C20.8326 5.44128 21.9089 6.00204 21.5484 8.06532C21.188 10.1286 14.795 15.1295 15.4171 16.2118C16.0391 17.2934 18.2312 14.9402 18.2312 14.9402C18.2312 14.9402 25.0907 8.49588 26.5842 10.1752C28.0776 11.8545 25.4512 13.2616 21.7082 15.6008C17.9646 17.9393 17.6744 18.557 18.2054 19.4418C18.7372 20.3266 26.9998 13.1351 27.7759 16.1838C28.5513 19.2324 19.3434 20.1173 19.9117 22.2219C20.48 24.3274 26.3979 18.2382 27.6082 20.6107C28.8193 22.9839 19.2574 25.7722 19.18 25.7929C16.0914 26.62 8.24723 28.3726 5.50686 24.2246Z" fill="#FFD21E"></path></svg>
	Community
	

	
		</a></div>
	
			


<div class="relative mb-1.5 flex flex-wrap gap-1.5 sm:flex-nowrap lg:mb-0"><div class="order-last sm:order-first"><div class="relative ">
	<button class="btn px-1.5 py-1.5 " type="button">
		
			<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" role="img" class="p-0.5" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 32 32"><circle cx="16" cy="7" r="3" fill="currentColor"></circle><circle cx="16" cy="16" r="3" fill="currentColor"></circle><circle cx="16" cy="25" r="3" fill="currentColor"></circle></svg>
		
		</button>
	
	
	</div></div>














	
		
		



</div>
	</div></div></header>
</div>
	
<div class="container relative flex flex-col md:grid md:space-y-0 w-full md:grid-cols-12  space-y-4 md:gap-6 mb-16"><section class="pt-8 border-gray-100 col-span-full"><div class="SVELTE_HYDRATER contents" data-target="ViewerHeader" data-props="{&quot;context&quot;:{&quot;repo&quot;:{&quot;name&quot;:&quot;anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4&quot;,&quot;type&quot;:&quot;model&quot;},&quot;rev&quot;:&quot;main&quot;,&quot;path&quot;:&quot;chat.py&quot;,&quot;subpaths&quot;:[{&quot;dir&quot;:&quot;chat.py&quot;}]},&quot;refs&quot;:{&quot;branches&quot;:[{&quot;name&quot;:&quot;main&quot;,&quot;ref&quot;:&quot;refs/heads/main&quot;,&quot;targetCommit&quot;:&quot;c6e39dbe127a2f9d79ac27a4065ae2457d492b86&quot;}],&quot;tags&quot;:[],&quot;converts&quot;:[]},&quot;view&quot;:&quot;blob&quot;}"><header class="flex flex-wrap items-center justify-start pb-2 md:justify-end lg:flex-nowrap"><div class="grow max-md:flex max-md:w-full max-md:items-start max-md:justify-between"><div class="relative mr-4 flex min-w-0 basis-auto flex-wrap items-center md:grow md:basis-full lg:basis-auto lg:flex-nowrap"><div class="relative mr-3 mb-2">
	<button class="text-sm md:text-base btn w-full cursor-pointer text-sm" type="button">
		<svg class="mr-1.5 text-gray-700 dark:text-gray-400" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" focusable="false" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24" style="transform: rotate(360deg);"><path d="M13 14c-3.36 0-4.46 1.35-4.82 2.24C9.25 16.7 10 17.76 10 19a3 3 0 0 1-3 3a3 3 0 0 1-3-3c0-1.31.83-2.42 2-2.83V7.83A2.99 2.99 0 0 1 4 5a3 3 0 0 1 3-3a3 3 0 0 1 3 3c0 1.31-.83 2.42-2 2.83v5.29c.88-.65 2.16-1.12 4-1.12c2.67 0 3.56-1.34 3.85-2.23A3.006 3.006 0 0 1 14 7a3 3 0 0 1 3-3a3 3 0 0 1 3 3c0 1.34-.88 2.5-2.09 2.86C17.65 11.29 16.68 14 13 14m-6 4a1 1 0 0 0-1 1a1 1 0 0 0 1 1a1 1 0 0 0 1-1a1 1 0 0 0-1-1M7 4a1 1 0 0 0-1 1a1 1 0 0 0 1 1a1 1 0 0 0 1-1a1 1 0 0 0-1-1m10 2a1 1 0 0 0-1 1a1 1 0 0 0 1 1a1 1 0 0 0 1-1a1 1 0 0 0-1-1z" fill="currentColor"></path></svg>
			main
		<svg class="-mr-1 text-gray-500" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24"><path d="M16.293 9.293L12 13.586L7.707 9.293l-1.414 1.414L12 16.414l5.707-5.707z" fill="currentColor"></path></svg></button>
	
	
	</div>
			<div class="relative mb-2 flex flex-wrap items-center"><a class="truncate text-gray-800 hover:underline" href="/anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4/tree/main">anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4</a>
				<span class="mx-1 text-gray-300">/</span>
					<span class="dark:text-gray-300">chat.py</span>
					<button class="text-xs ml-2 focus:outline-hidden inline-flex cursor-pointer items-center text-sm  mx-0.5   text-gray-600 " title="Copy path" type="button"><svg class="" xmlns="http://www.w3.org/2000/svg" aria-hidden="true" fill="currentColor" focusable="false" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 32 32"><path d="M28,10V28H10V10H28m0-2H10a2,2,0,0,0-2,2V28a2,2,0,0,0,2,2H28a2,2,0,0,0,2-2V10a2,2,0,0,0-2-2Z" transform="translate(0)"></path><path d="M4,18H2V4A2,2,0,0,1,4,2H18V4H4Z" transform="translate(0)"></path><rect fill="none" width="32" height="32"></rect></svg>
		</button></div></div>
		</div>
	
	</header></div>
			<div class="SVELTE_HYDRATER contents" data-target="LastCommit" data-props="{&quot;commitLast&quot;:{&quot;date&quot;:&quot;2025-07-07T20:18:52.000Z&quot;,&quot;verified&quot;:&quot;verified&quot;,&quot;subject&quot;:&quot;Upload folder using huggingface_hub&quot;,&quot;authors&quot;:[{&quot;_id&quot;:&quot;679d9680de0c0f8370cabcf3&quot;,&quot;avatar&quot;:&quot;https://cdn-avatars.huggingface.co/v1/production/uploads/679d9680de0c0f8370cabcf3/numfzR_Lto_Hkvk-Pj8l8.png&quot;,&quot;isHf&quot;:false,&quot;user&quot;:&quot;anemll&quot;}],&quot;commit&quot;:{&quot;id&quot;:&quot;dc049466e9548bc04b0a9ab7c04bbbfec898ac0e&quot;,&quot;parentIds&quot;:[&quot;85e6d2b2b3c9e77e61e71a819562a23ac51612ad&quot;]},&quot;title&quot;:&quot;Upload folder using huggingface_hub&quot;},&quot;repo&quot;:{&quot;name&quot;:&quot;anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4&quot;,&quot;type&quot;:&quot;model&quot;}}"><div class="from-gray-100-to-white bg-linear-to-t flex flex-wrap items-baseline gap-y-1 rounded-t-lg border border-b-0 px-3 py-2 dark:border-gray-800"><img class="mr-2.5 mt-0.5 h-4 w-4 self-center rounded-full" alt="anemll's picture" src="https://cdn-avatars.huggingface.co/v1/production/uploads/679d9680de0c0f8370cabcf3/numfzR_Lto_Hkvk-Pj8l8.png">
			<div class="mr-4 flex flex-none items-center truncate"><a class="hover:underline" href="/anemll">anemll
					</a>
				
			</div>
		<div class="mr-4 truncate font-mono text-xs text-gray-500 hover:prose-a:underline sm:text-sm"><!-- HTML_TAG_START -->Upload folder using huggingface_hub<!-- HTML_TAG_END --></div>
		<a class="rounded-sm border bg-gray-50 px-1.5 text-sm hover:underline dark:border-gray-800 dark:bg-gray-900" href="/anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4/commit/dc049466e9548bc04b0a9ab7c04bbbfec898ac0e">dc04946</a>
		<span class="mx-2 text-green-500 dark:text-green-600 px-1.5 border-green-100 dark:border-green-800 rounded-full border text-xs uppercase" title="This commit is signed and the signature is verified">verified</span>
		<time class="ml-auto hidden flex-none truncate pl-2 text-gray-500 dark:text-gray-400 lg:block" datetime="2025-07-07T20:18:52" title="Mon, 07 Jul 2025 20:18:52 GMT">2 months ago</time></div></div>
			<div class="relative flex flex-wrap items-center border px-3 py-1.5 text-sm text-gray-800 dark:border-gray-800 dark:bg-gray-900 ">
				<a class="my-1 mr-4 flex items-center hover:underline " href="/anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4/raw/main/chat.py"><svg class="mr-1.5" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" focusable="false" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 32 32" style="transform: rotate(360deg);"><path d="M31 16l-7 7l-1.41-1.41L28.17 16l-5.58-5.59L24 9l7 7z" fill="currentColor"></path><path d="M1 16l7-7l1.41 1.41L3.83 16l5.58 5.59L8 23l-7-7z" fill="currentColor"></path><path d="M12.419 25.484L17.639 6l1.932.518L14.35 26z" fill="currentColor"></path></svg>
							raw
						</a><div class="SVELTE_HYDRATER contents" data-target="CopyButton" data-props="{&quot;value&quot;:&quot;https://huggingface.co/anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4/resolve/main/chat.py&quot;,&quot;style&quot;:&quot;blank&quot;,&quot;label&quot;:&quot;Copy download link&quot;,&quot;classNames&quot;:&quot;my-1 mr-4 flex items-center no-underline hover:underline&quot;}"><button class="my-1 mr-4 flex items-center no-underline hover:underline       " title="Copy download link" type="button"><svg class="" xmlns="http://www.w3.org/2000/svg" aria-hidden="true" fill="currentColor" focusable="false" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 32 32"><path d="M28,10V28H10V10H28m0-2H10a2,2,0,0,0-2,2V28a2,2,0,0,0,2,2H28a2,2,0,0,0,2-2V10a2,2,0,0,0-2-2Z" transform="translate(0)"></path><path d="M4,18H2V4A2,2,0,0,1,4,2H18V4H4Z" transform="translate(0)"></path><rect fill="none" width="32" height="32"></rect></svg>
		<span class="ml-1.5 ">Copy download link</span></button></div><a class="my-1 mr-4 flex items-center hover:underline " href="/anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4/commits/main/chat.py"><svg class="mr-1.5" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" focusable="false" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 32 32" style="transform: rotate(360deg);"><path d="M16 4C9.383 4 4 9.383 4 16s5.383 12 12 12s12-5.383 12-12S22.617 4 16 4zm0 2c5.535 0 10 4.465 10 10s-4.465 10-10 10S6 21.535 6 16S10.465 6 16 6zm-1 2v9h7v-2h-5V8z" fill="currentColor"></path></svg>
							history
						</a><a class="my-1 mr-4 flex items-center hover:underline " href="/anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4/blame/main/chat.py"><svg class="mr-1.5" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" focusable="false" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 32 32" style="transform: rotate(360deg);"><path d="M16 2a14 14 0 1 0 14 14A14 14 0 0 0 16 2zm0 26a12 12 0 1 1 12-12a12 12 0 0 1-12 12z" fill="currentColor"></path><path d="M11.5 11a2.5 2.5 0 1 0 2.5 2.5a2.48 2.48 0 0 0-2.5-2.5z" fill="currentColor"></path><path d="M20.5 11a2.5 2.5 0 1 0 2.5 2.5a2.48 2.48 0 0 0-2.5-2.5z" fill="currentColor"></path></svg>
							blame
						</a><a class="my-1 mr-4 flex items-center hover:underline text-green-600 dark:text-green-500" href="/anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4/edit/main/chat.py"><svg class="mr-1.5" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" focusable="false" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 32 32"><path d="M2 26h28v2H2z" fill="currentColor"></path><path d="M25.4 9c.8-.8.8-2 0-2.8l-3.6-3.6c-.8-.8-2-.8-2.8 0l-15 15V24h6.4l15-15zm-5-5L24 7.6l-3 3L17.4 7l3-3zM6 22v-3.6l10-10l3.6 3.6l-10 10H6z" fill="currentColor"></path></svg>
							contribute
						</a><a class="my-1 mr-4 flex items-center hover:underline " href="/anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4/delete/main/chat.py"><svg class="mr-1.5" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" focusable="false" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 32 32"><path d="M12 12h2v12h-2z" fill="currentColor"></path><path d="M18 12h2v12h-2z" fill="currentColor"></path><path d="M4 6v2h2v20a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V8h2V6zm4 22V8h16v20z" fill="currentColor"></path><path d="M12 2h8v2h-8z" fill="currentColor"></path></svg>
							delete
						</a>

				<div class="mr-4 flex items-center"><div class="SVELTE_HYDRATER contents" data-target="ScanStatusBadge" data-props="{&quot;classNames&quot;:&quot;mr-2&quot;,&quot;scanStatus&quot;:{&quot;status&quot;:&quot;safe&quot;,&quot;protectAiScan&quot;:{&quot;status&quot;:&quot;unscanned&quot;,&quot;message&quot;:null,&quot;reportLink&quot;:&quot;https://protectai.com/insights/models/anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4/c6e39dbe127a2f9d79ac27a4065ae2457d492b86/files?blob-id=f35310fe3f96a3a2263fbac4f80c8aac543a39f9&amp;utm_source=huggingface&quot;},&quot;avScan&quot;:{&quot;status&quot;:&quot;safe&quot;,&quot;version&quot;:&quot;1.4.3/27745&quot;},&quot;pickleImportScan&quot;:{&quot;status&quot;:&quot;unscanned&quot;,&quot;pickleImports&quot;:[],&quot;version&quot;:&quot;0.0.0&quot;},&quot;jFrogScan&quot;:{&quot;status&quot;:&quot;unscanned&quot;,&quot;message&quot;:&quot;Not a machine-learning model&quot;,&quot;reportLink&quot;:&quot;&quot;,&quot;reportLabel&quot;:&quot;&quot;}},&quot;repo&quot;:{&quot;name&quot;:&quot;anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4&quot;,&quot;type&quot;:&quot;model&quot;},&quot;revision&quot;:&quot;main&quot;,&quot;filePath&quot;:&quot;chat.py&quot;,&quot;openByDefault&quot;:false}"><div class="sm:relative mr-2"><button class="flex h-[1.125rem] select-none items-center gap-0.5 rounded border pl-0.5 pr-0.5 text-xs leading-tight text-gray-400 hover:cursor-pointer text-gray-400 hover:border-gray-200 hover:bg-gray-50 hover:text-gray-500 dark:border-gray-800 dark:hover:bg-gray-800 dark:hover:text-gray-200 "><svg class="flex-none" width="1em" height="1em" viewBox="0 0 22 28" fill="none" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" clip-rule="evenodd" d="M15.3634 10.3639C15.8486 10.8491 15.8486 11.6357 15.3634 12.1209L10.9292 16.5551C10.6058 16.8785 10.0814 16.8785 9.7579 16.5551L7.03051 13.8277C6.54532 13.3425 6.54532 12.5558 7.03051 12.0707C7.51569 11.5855 8.30234 11.5855 8.78752 12.0707L9.7579 13.041C10.0814 13.3645 10.6058 13.3645 10.9292 13.041L13.6064 10.3639C14.0916 9.8787 14.8782 9.8787 15.3634 10.3639Z" fill="currentColor"></path><path fill-rule="evenodd" clip-rule="evenodd" d="M10.6666 27.12C4.93329 25.28 0 19.2267 0 12.7867V6.52001C0 5.40001 0.693334 4.41334 1.73333 4.01334L9.73333 1.01334C10.3333 0.786673 11 0.786673 11.6 1.02667L19.6 4.02667C20.1083 4.21658 20.5465 4.55701 20.8562 5.00252C21.1659 5.44803 21.3324 5.97742 21.3333 6.52001V12.7867C21.3333 19.24 16.4 25.28 10.6666 27.12Z" fill="currentColor" fill-opacity="0.22"></path><path d="M10.0845 1.94967L10.0867 1.94881C10.4587 1.8083 10.8666 1.81036 11.2286 1.95515L11.2387 1.95919L11.2489 1.963L19.2489 4.963L19.25 4.96342C19.5677 5.08211 19.8416 5.29488 20.0351 5.57333C20.2285 5.85151 20.3326 6.18203 20.3333 6.52082C20.3333 6.52113 20.3333 6.52144 20.3333 6.52176L20.3333 12.7867C20.3333 18.6535 15.8922 24.2319 10.6666 26.0652C5.44153 24.2316 1 18.6409 1 12.7867V6.52001C1 5.82357 1.42893 5.20343 2.08883 4.94803L10.0845 1.94967Z" stroke="currentColor" stroke-opacity="0.30" stroke-width="2"></path></svg>

			<span class="mr-0.5 max-sm:hidden">Safe</span></button>

	</div></div>
						</div>

				<div class="flex items-center gap-x-3 dark:text-gray-300 sm:ml-auto"><div class="SVELTE_HYDRATER contents" data-target="LineWrapButton" data-props="{&quot;classNames&quot;:&quot;text-xs&quot;,&quot;lineSelectorClass&quot;:&quot;blob-line&quot;}">

<button class="text-xs focus:outline-hidden inline-flex cursor-pointer items-center justify-center text-sm  mx-0.5  " type="button"><svg class="opacity-40" width="1em" height="1em" viewBox="0 0 12 11" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M0.75 1.25H11.25M0.75 5H9C9.75 5 11.25 5.375 11.25 6.875C11.25 8.375 9.99975 8.75 9.375 8.75H6M6 8.75L7.5 7.25M6 8.75L7.5 10.25M0.75 8.75H3.75" stroke="currentColor" stroke-width="1.125" stroke-linecap="round" stroke-linejoin="round"></path></svg></button></div>
					45.1 kB</div></div>

			<div class="relative min-h-[100px] overflow-hidden rounded-b-lg border border-t-0 leading-tight dark:border-gray-800 dark:bg-gray-925">
				<div class="py-3"><div class="SVELTE_HYDRATER contents" data-target="BlobContent" data-props="{&quot;lines&quot;:[&quot;<span class=\&quot;hljs-comment\&quot;># chat.py</span>&quot;,&quot;<span class=\&quot;hljs-comment\&quot;>#!/usr/bin/env python3</span>&quot;,&quot;<span class=\&quot;hljs-comment\&quot;># chat.py</span>&quot;,&quot;<span class=\&quot;hljs-comment\&quot;># Copyright (c) 2025 Anemll</span>&quot;,&quot;<span class=\&quot;hljs-comment\&quot;># Licensed under the MIT License</span>&quot;,&quot;&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>import</span> argparse&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>import</span> os&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>import</span> re&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>import</span> glob&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>from</span> pathlib <span class=\&quot;hljs-keyword\&quot;>import</span> Path&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>import</span> coremltools <span class=\&quot;hljs-keyword\&quot;>as</span> ct&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>from</span> transformers <span class=\&quot;hljs-keyword\&quot;>import</span> LlamaTokenizer, AutoTokenizer&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>import</span> torch&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>import</span> torch.nn.functional <span class=\&quot;hljs-keyword\&quot;>as</span> F&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>import</span> numpy <span class=\&quot;hljs-keyword\&quot;>as</span> np&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>import</span> queue&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>import</span> threading&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>import</span> time&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>import</span> yaml&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>import</span> sys&quot;,&quot;&quot;,&quot;<span class=\&quot;hljs-comment\&quot;># ANSI color codes</span>&quot;,&quot;LIGHT_BLUE = <span class=\&quot;hljs-string\&quot;>&amp;quot;\\033[94m&amp;quot;</span>&quot;,&quot;DARK_BLUE = <span class=\&quot;hljs-string\&quot;>&amp;quot;\\033[34m&amp;quot;</span>&quot;,&quot;LIGHT_GREEN = <span class=\&quot;hljs-string\&quot;>&amp;quot;\\033[92m&amp;quot;</span>&quot;,&quot;RESET_COLOR = <span class=\&quot;hljs-string\&quot;>&amp;quot;\\033[0m&amp;quot;</span>&quot;,&quot;&quot;,&quot;<span class=\&quot;hljs-comment\&quot;># Add at top with other constants</span>&quot;,&quot;WARMUP_TOKEN_LIMIT = <span class=\&quot;hljs-number\&quot;>10</span>  <span class=\&quot;hljs-comment\&quot;># Maximum tokens to generate during warmup</span>&quot;,&quot;&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>class</span> <span class=\&quot;hljs-title class_\&quot;>TokenPrinter</span>:&quot;,&quot;    <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;quot;&amp;quot;Handles background printing of generated tokens.&amp;quot;&amp;quot;&amp;quot;</span>&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>__init__</span>(<span class=\&quot;hljs-params\&quot;>self, tokenizer</span>):&quot;,&quot;        self.tokenizer = tokenizer&quot;,&quot;        self.token_queue = queue.Queue()&quot;,&quot;        self.stop_event = threading.Event()&quot;,&quot;        self.thread = <span class=\&quot;hljs-literal\&quot;>None</span>&quot;,&quot;        self.buffer = <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;quot;</span>&quot;,&quot;        self.lock = threading.Lock()&quot;,&quot;        self.thinking = <span class=\&quot;hljs-literal\&quot;>True</span>  <span class=\&quot;hljs-comment\&quot;># Track if we&amp;#x27;re still in thinking mode</span>&quot;,&quot;        self.decoding_buffer = []  <span class=\&quot;hljs-comment\&quot;># Buffer for token IDs</span>&quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Add token counting and timing</span>&quot;,&quot;        self.start_time = time.time()&quot;,&quot;        self.token_count = <span class=\&quot;hljs-number\&quot;>0</span>&quot;,&quot;        self.start()&quot;,&quot;&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>start</span>(<span class=\&quot;hljs-params\&quot;>self</span>):&quot;,&quot;        <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;quot;&amp;quot;Start the printer thread.&amp;quot;&amp;quot;&amp;quot;</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> self.thread <span class=\&quot;hljs-keyword\&quot;>is</span> <span class=\&quot;hljs-literal\&quot;>None</span>:&quot;,&quot;            self.thread = threading.Thread(target=self._print_worker)&quot;,&quot;            self.thread.daemon = <span class=\&quot;hljs-literal\&quot;>True</span>&quot;,&quot;            self.thread.start()&quot;,&quot;&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>add_token</span>(<span class=\&quot;hljs-params\&quot;>self, token_id</span>):&quot;,&quot;        <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;quot;&amp;quot;Add a token to the print queue.&amp;quot;&amp;quot;&amp;quot;</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> self.stop_event.is_set():&quot;,&quot;            self.token_queue.put(token_id)&quot;,&quot;            self.token_count += <span class=\&quot;hljs-number\&quot;>1</span>&quot;,&quot;&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>drain_buffer</span>(<span class=\&quot;hljs-params\&quot;>self, eval_mode=<span class=\&quot;hljs-literal\&quot;>False</span></span>):&quot;,&quot;        <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;quot;&amp;quot;Decode token IDs from decoding_buffer in the main thread.&amp;quot;&amp;quot;&amp;quot;</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> self.decoding_buffer:&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>return</span>&quot;,&quot;&quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Decode all tokens at once in the main thread</span>&quot;,&quot;        token_str = self.tokenizer.decode(self.decoding_buffer)&quot;,&quot;        self.decoding_buffer.clear()&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Store the text in buffer for later saving to file</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>with</span> self.lock:&quot;,&quot;            self.buffer += token_str&quot;,&quot;&quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Skip printing in eval mode</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> eval_mode:&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>return</span>&quot;,&quot;&quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Color-handling logic</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> self.thinking <span class=\&quot;hljs-keyword\&quot;>and</span> <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;lt;/think&amp;gt;&amp;quot;</span> <span class=\&quot;hljs-keyword\&quot;>in</span> token_str:&quot;,&quot;            self.thinking = <span class=\&quot;hljs-literal\&quot;>False</span>&quot;,&quot;            parts = token_str.split(<span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;lt;/think&amp;gt;&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-built_in\&quot;>len</span>(parts) &amp;gt; <span class=\&quot;hljs-number\&quot;>0</span>:&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(parts[<span class=\&quot;hljs-number\&quot;>0</span>] + <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;lt;/think&amp;gt;&amp;quot;</span>, end=<span class=\&quot;hljs-string\&quot;>&amp;#x27;&amp;#x27;</span>, flush=<span class=\&quot;hljs-literal\&quot;>True</span>)&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-built_in\&quot;>len</span>(parts) &amp;gt; <span class=\&quot;hljs-number\&quot;>1</span>:&quot;,&quot;                    <span class=\&quot;hljs-built_in\&quot;>print</span>(LIGHT_BLUE + parts[<span class=\&quot;hljs-number\&quot;>1</span>], end=<span class=\&quot;hljs-string\&quot;>&amp;#x27;&amp;#x27;</span>, flush=<span class=\&quot;hljs-literal\&quot;>True</span>)&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> self.thinking:&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(LIGHT_BLUE + token_str, end=<span class=\&quot;hljs-string\&quot;>&amp;#x27;&amp;#x27;</span>, flush=<span class=\&quot;hljs-literal\&quot;>True</span>)&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(token_str, end=<span class=\&quot;hljs-string\&quot;>&amp;#x27;&amp;#x27;</span>, flush=<span class=\&quot;hljs-literal\&quot;>True</span>)&quot;,&quot;&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>_print_worker</span>(<span class=\&quot;hljs-params\&quot;>self</span>):&quot;,&quot;        <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;quot;&amp;quot;Worker thread that takes token_ids from the queue.&amp;quot;&amp;quot;&amp;quot;</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>while</span> <span class=\&quot;hljs-keyword\&quot;>not</span> self.stop_event.is_set():&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>try</span>:&quot;,&quot;                token_id = self.token_queue.get(timeout=<span class=\&quot;hljs-number\&quot;>0.01</span>)&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>with</span> self.lock:&quot;,&quot;                    self.decoding_buffer.append(token_id)&quot;,&quot;                self.token_queue.task_done()&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>except</span> queue.Empty:&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>continue</span>&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>except</span> Exception <span class=\&quot;hljs-keyword\&quot;>as</span> e:&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nError: Token printer error: <span class=\&quot;hljs-subst\&quot;>{<span class=\&quot;hljs-built_in\&quot;>str</span>(e)}</span>&amp;quot;</span>)&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>break</span>&quot;,&quot;&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>stop</span>(<span class=\&quot;hljs-params\&quot;>self, eval_mode=<span class=\&quot;hljs-literal\&quot;>False</span></span>):&quot;,&quot;        <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;quot;&amp;quot;Stop the printer thread.&amp;quot;&amp;quot;&amp;quot;</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> self.thread <span class=\&quot;hljs-keyword\&quot;>and</span> self.thread.is_alive():&quot;,&quot;            <span class=\&quot;hljs-comment\&quot;># Ensure any remaining tokens are processed</span>&quot;,&quot;            self.drain_buffer()&quot;,&quot;            self.stop_event.<span class=\&quot;hljs-built_in\&quot;>set</span>()&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>try</span>:&quot;,&quot;                self.thread.join(timeout=<span class=\&quot;hljs-number\&quot;>1.0</span>)&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>except</span> Exception:&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>pass</span>&quot;,&quot;            <span class=\&quot;hljs-comment\&quot;># Calculate and print tokens/s with shorter format in blue (unless in eval mode)</span>&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> eval_mode:&quot;,&quot;                elapsed = time.time() - self.start_time&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>if</span> elapsed &amp;gt; <span class=\&quot;hljs-number\&quot;>0</span> <span class=\&quot;hljs-keyword\&quot;>and</span> self.token_count &amp;gt; <span class=\&quot;hljs-number\&quot;>0</span>:&quot;,&quot;                    tokens_per_sec = self.token_count / elapsed&quot;,&quot;                    <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\n<span class=\&quot;hljs-subst\&quot;>{DARK_BLUE}</span><span class=\&quot;hljs-subst\&quot;>{tokens_per_sec:<span class=\&quot;hljs-number\&quot;>.1</span>f}</span> t/s<span class=\&quot;hljs-subst\&quot;>{RESET_COLOR}</span>&amp;quot;</span>)&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;                    <span class=\&quot;hljs-built_in\&quot;>print</span>(RESET_COLOR)  <span class=\&quot;hljs-comment\&quot;># Reset color at the end</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>return</span> self.buffer&quot;,&quot;&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>parse_model_path</span>(<span class=\&quot;hljs-params\&quot;>path</span>):&quot;,&quot;    <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;quot;&amp;quot;Parse model path and return full path with .mlmodelc or .mlpackage extension.&amp;quot;&amp;quot;&amp;quot;</span>&quot;,&quot;    path = Path(path)&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># If path exists exactly as specified, return it</span>&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>if</span> path.exists():&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>return</span> <span class=\&quot;hljs-built_in\&quot;>str</span>(path)&quot;,&quot;        &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Try with both extensions</span>&quot;,&quot;    candidates = [&quot;,&quot;        path,  <span class=\&quot;hljs-comment\&quot;># Original path</span>&quot;,&quot;        path.with_suffix(<span class=\&quot;hljs-string\&quot;>&amp;#x27;.mlmodelc&amp;#x27;</span>),  <span class=\&quot;hljs-comment\&quot;># With .mlmodelc</span>&quot;,&quot;        path.with_suffix(<span class=\&quot;hljs-string\&quot;>&amp;#x27;.mlpackage&amp;#x27;</span>),  <span class=\&quot;hljs-comment\&quot;># With .mlpackage</span>&quot;,&quot;        Path(<span class=\&quot;hljs-built_in\&quot;>str</span>(path) + <span class=\&quot;hljs-string\&quot;>&amp;#x27;.mlmodelc&amp;#x27;</span>),  <span class=\&quot;hljs-comment\&quot;># Handle case where extension is included</span>&quot;,&quot;        Path(<span class=\&quot;hljs-built_in\&quot;>str</span>(path) + <span class=\&quot;hljs-string\&quot;>&amp;#x27;.mlpackage&amp;#x27;</span>)&quot;,&quot;    ]&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Try all possible paths</span>&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>for</span> candidate <span class=\&quot;hljs-keyword\&quot;>in</span> candidates:&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> candidate.exists():&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>return</span> <span class=\&quot;hljs-built_in\&quot;>str</span>(candidate)&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># If embeddings with LUT suffix not found, try without LUT suffix</span>&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-string\&quot;>&amp;quot;_lut&amp;quot;</span> <span class=\&quot;hljs-keyword\&quot;>in</span> <span class=\&quot;hljs-built_in\&quot;>str</span>(path) <span class=\&quot;hljs-keyword\&quot;>and</span> <span class=\&quot;hljs-string\&quot;>&amp;quot;embeddings&amp;quot;</span> <span class=\&quot;hljs-keyword\&quot;>in</span> <span class=\&quot;hljs-built_in\&quot;>str</span>(path):&quot;,&quot;        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;Failed to find <span class=\&quot;hljs-subst\&quot;>{path}</span>, trying without LUT suffix...&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Remove LUT suffix</span>&quot;,&quot;        path_no_lut = <span class=\&quot;hljs-built_in\&quot;>str</span>(path).split(<span class=\&quot;hljs-string\&quot;>&amp;quot;_lut&amp;quot;</span>)[<span class=\&quot;hljs-number\&quot;>0</span>]&quot;,&quot;        path_no_lut = Path(path_no_lut)&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Try candidates without LUT suffix</span>&quot;,&quot;        candidates_no_lut = [&quot;,&quot;            path_no_lut,&quot;,&quot;            path_no_lut.with_suffix(<span class=\&quot;hljs-string\&quot;>&amp;#x27;.mlmodelc&amp;#x27;</span>),&quot;,&quot;            path_no_lut.with_suffix(<span class=\&quot;hljs-string\&quot;>&amp;#x27;.mlpackage&amp;#x27;</span>),&quot;,&quot;            Path(<span class=\&quot;hljs-built_in\&quot;>str</span>(path_no_lut) + <span class=\&quot;hljs-string\&quot;>&amp;#x27;.mlmodelc&amp;#x27;</span>),&quot;,&quot;            Path(<span class=\&quot;hljs-built_in\&quot;>str</span>(path_no_lut) + <span class=\&quot;hljs-string\&quot;>&amp;#x27;.mlpackage&amp;#x27;</span>)&quot;,&quot;        ]&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>for</span> candidate <span class=\&quot;hljs-keyword\&quot;>in</span> candidates_no_lut:&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> candidate.exists():&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>return</span> <span class=\&quot;hljs-built_in\&quot;>str</span>(candidate)&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Add no-LUT candidates to the list for error reporting</span>&quot;,&quot;        candidates.extend(candidates_no_lut)&quot;,&quot;            &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># If we get here, no valid path was found</span>&quot;,&quot;    <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nError: Model not found. Tried following paths:&amp;quot;</span>)&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>for</span> candidate <span class=\&quot;hljs-keyword\&quot;>in</span> candidates:&quot;,&quot;        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  <span class=\&quot;hljs-subst\&quot;>{candidate}</span>&amp;quot;</span>)&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>raise</span> FileNotFoundError(<span class=\&quot;hljs-string\&quot;>f&amp;quot;Model not found: <span class=\&quot;hljs-subst\&quot;>{path}</span>&amp;quot;</span>)&quot;,&quot;&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>parse_ffn_filename</span>(<span class=\&quot;hljs-params\&quot;>path</span>):&quot;,&quot;    <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;quot;&amp;quot;Parse FFN model filename to extract chunk information.&amp;quot;&amp;quot;&amp;quot;</span>&quot;,&quot;    path = Path(path)&quot;,&quot;    pattern = <span class=\&quot;hljs-string\&quot;>r&amp;#x27;FFN_PF.*_chunk_(\\d+)of(\\d+)&amp;#x27;</span>&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>match</span> = re.search(pattern, path.name)&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>match</span>:&quot;,&quot;        current_chunk = <span class=\&quot;hljs-built_in\&quot;>int</span>(<span class=\&quot;hljs-keyword\&quot;>match</span>.group(<span class=\&quot;hljs-number\&quot;>1</span>))&quot;,&quot;        total_chunks = <span class=\&quot;hljs-built_in\&quot;>int</span>(<span class=\&quot;hljs-keyword\&quot;>match</span>.group(<span class=\&quot;hljs-number\&quot;>2</span>))&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>return</span> current_chunk, total_chunks&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>return</span> <span class=\&quot;hljs-literal\&quot;>None</span>, <span class=\&quot;hljs-literal\&quot;>None</span>&quot;,&quot;&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>find_all_chunks</span>(<span class=\&quot;hljs-params\&quot;>base_path</span>):&quot;,&quot;    <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;quot;&amp;quot;Find all chunk files matching the base FFN path pattern.&amp;quot;&amp;quot;&amp;quot;</span>&quot;,&quot;    path = Path(base_path)&quot;,&quot;    pattern = re.sub(<span class=\&quot;hljs-string\&quot;>r&amp;#x27;_chunk_\\d+of\\d+&amp;#x27;</span>, <span class=\&quot;hljs-string\&quot;>&amp;#x27;_chunk_*&amp;#x27;</span>, <span class=\&quot;hljs-built_in\&quot;>str</span>(path))&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>return</span> <span class=\&quot;hljs-built_in\&quot;>sorted</span>(glob.glob(pattern))&quot;,&quot;&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>load_model</span>(<span class=\&quot;hljs-params\&quot;>path, function_name=<span class=\&quot;hljs-literal\&quot;>None</span></span>):&quot;,&quot;    <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;quot;&amp;quot;Load a CoreML model, handling both .mlmodelc and .mlpackage formats.&amp;quot;&amp;quot;&amp;quot;</span>&quot;,&quot;    path = Path(path)&quot;,&quot;    compute_unit = ct.ComputeUnit.CPU_AND_NE&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>try</span>:&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> path.suffix == <span class=\&quot;hljs-string\&quot;>&amp;#x27;.mlmodelc&amp;#x27;</span>:&quot;,&quot;            <span class=\&quot;hljs-comment\&quot;># For compiled models (.mlmodelc), use CompiledMLModel</span>&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> function_name:&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>return</span> ct.models.CompiledMLModel(<span class=\&quot;hljs-built_in\&quot;>str</span>(path), compute_unit, function_name=function_name)&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>return</span> ct.models.CompiledMLModel(<span class=\&quot;hljs-built_in\&quot;>str</span>(path), compute_unit)&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;            <span class=\&quot;hljs-comment\&quot;># For packages (.mlpackage)</span>&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> function_name:&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>return</span> ct.models.MLModel(<span class=\&quot;hljs-built_in\&quot;>str</span>(path), function_name=function_name)&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>return</span> ct.models.MLModel(<span class=\&quot;hljs-built_in\&quot;>str</span>(path))&quot;,&quot;                &quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>except</span> RuntimeError <span class=\&quot;hljs-keyword\&quot;>as</span> e:&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-string\&quot;>&amp;quot;valid manifest does not exist&amp;quot;</span> <span class=\&quot;hljs-keyword\&quot;>in</span> <span class=\&quot;hljs-built_in\&quot;>str</span>(e):&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nError: Could not load compiled model at <span class=\&quot;hljs-subst\&quot;>{path}</span>&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;This might be because:&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;1. The model is not properly compiled&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;2. The model was compiled for a different OS version&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;3. The model needs to be recompiled&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nTry using the .mlpackage version instead, or recompile the model.&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>raise</span>&quot;,&quot;&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>load_metadata</span>(<span class=\&quot;hljs-params\&quot;>model,args</span>):&quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Extract metadata and config parameters</span>&quot;,&quot;    metadata = {}&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-built_in\&quot;>hasattr</span>(model, <span class=\&quot;hljs-string\&quot;>&amp;#x27;user_defined_metadata&amp;#x27;</span>):&quot;,&quot;        meta = model.user_defined_metadata&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Extract key parameters with defaults</span>&quot;,&quot;        metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;context_length&amp;#x27;</span>] = <span class=\&quot;hljs-built_in\&quot;>int</span>(meta.get(<span class=\&quot;hljs-string\&quot;>&amp;#x27;com.anemll.context_length&amp;#x27;</span>, <span class=\&quot;hljs-number\&quot;>512</span>))&quot;,&quot;        metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;state_length&amp;#x27;</span>] = <span class=\&quot;hljs-built_in\&quot;>int</span>(meta.get(<span class=\&quot;hljs-string\&quot;>&amp;#x27;com.anemll.state_length&amp;#x27;</span>, metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;context_length&amp;#x27;</span>]))  <span class=\&quot;hljs-comment\&quot;># Added state_length</span>&quot;,&quot;        metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;batch_size&amp;#x27;</span>] = <span class=\&quot;hljs-built_in\&quot;>int</span>(meta.get(<span class=\&quot;hljs-string\&quot;>&amp;#x27;com.anemll.batch_size&amp;#x27;</span>, <span class=\&quot;hljs-number\&quot;>64</span>))&quot;,&quot;        metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;lut_bits&amp;#x27;</span>] = <span class=\&quot;hljs-built_in\&quot;>int</span>(meta.get(<span class=\&quot;hljs-string\&quot;>&amp;#x27;com.anemll.lut_bits&amp;#x27;</span>, <span class=\&quot;hljs-number\&quot;>0</span>))&quot;,&quot;        metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;num_chunks&amp;#x27;</span>] = <span class=\&quot;hljs-built_in\&quot;>int</span>(meta.get(<span class=\&quot;hljs-string\&quot;>&amp;#x27;com.anemll.num_chunks&amp;#x27;</span>, <span class=\&quot;hljs-number\&quot;>1</span>))&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nExtracted Parameters:&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  Context Length: <span class=\&quot;hljs-subst\&quot;>{metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;context_length&amp;#x27;</span>]}</span>&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  State Length: <span class=\&quot;hljs-subst\&quot;>{metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;state_length&amp;#x27;</span>]}</span>&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  Prefill Batch Size: <span class=\&quot;hljs-subst\&quot;>{metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;batch_size&amp;#x27;</span>]}</span>&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  LUT Bits: <span class=\&quot;hljs-subst\&quot;>{metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;lut_bits&amp;#x27;</span>]}</span>&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  Number of Chunks: <span class=\&quot;hljs-subst\&quot;>{metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;num_chunks&amp;#x27;</span>]}</span>&amp;quot;</span>)&quot;,&quot;            &quot;,&quot;            <span class=\&quot;hljs-comment\&quot;># Print model info</span>&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nModel Info:&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-string\&quot;>&amp;#x27;com.anemll.info&amp;#x27;</span> <span class=\&quot;hljs-keyword\&quot;>in</span> meta:&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  <span class=\&quot;hljs-subst\&quot;>{meta[<span class=\&quot;hljs-string\&quot;>&amp;#x27;com.anemll.info&amp;#x27;</span>]}</span>&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-string\&quot;>&amp;#x27;com.github.apple.coremltools.version&amp;#x27;</span> <span class=\&quot;hljs-keyword\&quot;>in</span> meta:&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  CoreML Tools: <span class=\&quot;hljs-subst\&quot;>{meta[<span class=\&quot;hljs-string\&quot;>&amp;#x27;com.github.apple.coremltools.version&amp;#x27;</span>]}</span>&amp;quot;</span>)&quot;,&quot;            &quot;,&quot;            <span class=\&quot;hljs-comment\&quot;># Print model input/output shapes</span>&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nModel Shapes:&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-built_in\&quot;>hasattr</span>(model, <span class=\&quot;hljs-string\&quot;>&amp;#x27;input_description&amp;#x27;</span>):&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;  Inputs:&amp;quot;</span>)&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>try</span>:&quot;,&quot;                    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-built_in\&quot;>hasattr</span>(model.input_description, <span class=\&quot;hljs-string\&quot;>&amp;#x27;items&amp;#x27;</span>):&quot;,&quot;                        <span class=\&quot;hljs-keyword\&quot;>for</span> name, desc <span class=\&quot;hljs-keyword\&quot;>in</span> model.input_description.items():&quot;,&quot;                            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;    <span class=\&quot;hljs-subst\&quot;>{name}</span>: <span class=\&quot;hljs-subst\&quot;>{desc}</span>&amp;quot;</span>)&quot;,&quot;                    <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;                        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;    <span class=\&quot;hljs-subst\&quot;>{model.input_description}</span>&amp;quot;</span>)&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>except</span>:&quot;,&quot;                    <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;    Input description: <span class=\&quot;hljs-subst\&quot;>{<span class=\&quot;hljs-built_in\&quot;>type</span>(model.input_description)}</span>&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-built_in\&quot;>hasattr</span>(model, <span class=\&quot;hljs-string\&quot;>&amp;#x27;output_description&amp;#x27;</span>):&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;  Outputs:&amp;quot;</span>)&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>try</span>:&quot;,&quot;                    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-built_in\&quot;>hasattr</span>(model.output_description, <span class=\&quot;hljs-string\&quot;>&amp;#x27;items&amp;#x27;</span>):&quot;,&quot;                        <span class=\&quot;hljs-keyword\&quot;>for</span> name, desc <span class=\&quot;hljs-keyword\&quot;>in</span> model.output_description.items():&quot;,&quot;                            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;    <span class=\&quot;hljs-subst\&quot;>{name}</span>: <span class=\&quot;hljs-subst\&quot;>{desc}</span>&amp;quot;</span>)&quot;,&quot;                    <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;                        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;    <span class=\&quot;hljs-subst\&quot;>{model.output_description}</span>&amp;quot;</span>)&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>except</span>:&quot;,&quot;                    <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;    Output description: <span class=\&quot;hljs-subst\&quot;>{<span class=\&quot;hljs-built_in\&quot;>type</span>(model.output_description)}</span>&amp;quot;</span>)&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nWarning: No metadata found in model&amp;quot;</span>)&quot;,&quot;&quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Check if model directory name contains context length pattern (ctxXXX)</span>&quot;,&quot;        ctx_len = <span class=\&quot;hljs-number\&quot;>512</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> args.context_length <span class=\&quot;hljs-keyword\&quot;>is</span>  <span class=\&quot;hljs-literal\&quot;>None</span>:&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>import</span> re&quot;,&quot;            ctx_match = re.search(<span class=\&quot;hljs-string\&quot;>r&amp;#x27;ctx(\\d+)&amp;#x27;</span>, <span class=\&quot;hljs-built_in\&quot;>str</span>(args.d))&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> ctx_match:&quot;,&quot;                ctx_len0 = <span class=\&quot;hljs-built_in\&quot;>int</span>(ctx_match.group(<span class=\&quot;hljs-number\&quot;>1</span>))&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-number\&quot;>512</span> &amp;lt;= ctx_len0 &amp;lt;= <span class=\&quot;hljs-number\&quot;>8096</span>:&quot;,&quot;                    ctx_len = ctx_len0&quot;,&quot;                    <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nDetected context length <span class=\&quot;hljs-subst\&quot;>{ctx_len}</span> from directory name&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nWarning: No context length found in directory  <span class=\&quot;hljs-subst\&quot;>{ctx_len}</span> from directory name <span class=\&quot;hljs-subst\&quot;>{args.d}</span>&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;            ctx_len = args.context_length&quot;,&quot;&quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Use defaults or values from args</span>&quot;,&quot;        metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;context_length&amp;#x27;</span>] = ctx_len&quot;,&quot;        metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;state_length&amp;#x27;</span>] = ctx_len&quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Get batch size from args or use default</span>&quot;,&quot;        metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;batch_size&amp;#x27;</span>] = <span class=\&quot;hljs-built_in\&quot;>getattr</span>(args, <span class=\&quot;hljs-string\&quot;>&amp;#x27;batch_size&amp;#x27;</span>, <span class=\&quot;hljs-number\&quot;>64</span>)&quot;,&quot;        metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;lut_bits&amp;#x27;</span>] = <span class=\&quot;hljs-number\&quot;>4</span>&quot;,&quot;        metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;num_chunks&amp;#x27;</span>] = <span class=\&quot;hljs-built_in\&quot;>getattr</span>(args, <span class=\&quot;hljs-string\&quot;>&amp;#x27;num_chunks&amp;#x27;</span>, <span class=\&quot;hljs-number\&quot;>4</span>)&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nUsing parameters:&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  Context Length: <span class=\&quot;hljs-subst\&quot;>{metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;context_length&amp;#x27;</span>]}</span>&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  State Length: <span class=\&quot;hljs-subst\&quot;>{metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;state_length&amp;#x27;</span>]}</span>&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  Prefill Batch Size: <span class=\&quot;hljs-subst\&quot;>{metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;batch_size&amp;#x27;</span>]}</span>&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  LUT Bits: <span class=\&quot;hljs-subst\&quot;>{metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;lut_bits&amp;#x27;</span>]}</span>&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  Number of Chunks: <span class=\&quot;hljs-subst\&quot;>{metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;num_chunks&amp;#x27;</span>]}</span>&amp;quot;</span>)&quot;,&quot;&quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Override with values from args if they exist</span>&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-built_in\&quot;>hasattr</span>(args, <span class=\&quot;hljs-string\&quot;>&amp;#x27;batch_size&amp;#x27;</span>) <span class=\&quot;hljs-keyword\&quot;>and</span> args.batch_size <span class=\&quot;hljs-keyword\&quot;>is</span> <span class=\&quot;hljs-keyword\&quot;>not</span> <span class=\&quot;hljs-literal\&quot;>None</span>:&quot;,&quot;        metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;batch_size&amp;#x27;</span>] = args.batch_size&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nOverriding batch size from args: <span class=\&quot;hljs-subst\&quot;>{args.batch_size}</span>&amp;quot;</span>)&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-built_in\&quot;>hasattr</span>(args, <span class=\&quot;hljs-string\&quot;>&amp;#x27;num_chunks&amp;#x27;</span>) <span class=\&quot;hljs-keyword\&quot;>and</span> args.num_chunks <span class=\&quot;hljs-keyword\&quot;>is</span> <span class=\&quot;hljs-keyword\&quot;>not</span> <span class=\&quot;hljs-literal\&quot;>None</span>:&quot;,&quot;        metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;num_chunks&amp;#x27;</span>] = args.num_chunks&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nOverriding num chunks from args: <span class=\&quot;hljs-subst\&quot;>{args.num_chunks}</span>&amp;quot;</span>)&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>return</span> metadata&quot;,&quot;    &quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>load_models</span>(<span class=\&quot;hljs-params\&quot;>args,metadata</span>):&quot;,&quot;    <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;quot;&amp;quot;Load all required models and extract metadata.&amp;quot;&amp;quot;&amp;quot;</span>&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nLoading models...&amp;quot;</span>)&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>try</span>:&quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Load embeddings model</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nLoading embeddings model...&amp;quot;</span>)&quot;,&quot;        embed_path = parse_model_path(args.embed)&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;Loading from: <span class=\&quot;hljs-subst\&quot;>{embed_path}</span>&amp;quot;</span>)&quot;,&quot;        embed_model = load_model(embed_path)&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;Embeddings model loaded successfully&amp;quot;</span>)&quot;,&quot;        metadata = load_metadata(embed_model,args)&quot;,&quot;        &quot;,&quot;&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Load LM head model</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nLoading LM head model...&amp;quot;</span>)&quot;,&quot;        lmhead_path = parse_model_path(args.lmhead)&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;Loading from: <span class=\&quot;hljs-subst\&quot;>{lmhead_path}</span>&amp;quot;</span>)&quot;,&quot;        lmhead_model = load_model(lmhead_path)&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;LM head model loaded successfully&amp;quot;</span>)&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Parse FFN path and find chunks if needed</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nLoading FFN+PREFILL model(s)...&amp;quot;</span>)&quot;,&quot;        ffn_path = parse_model_path(args.ffn)&quot;,&quot;        chunk_no, total_chunks = parse_ffn_filename(ffn_path)&quot;,&quot;        &quot;,&quot;        ffn_models = []&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> chunk_no <span class=\&quot;hljs-keyword\&quot;>and</span> total_chunks:&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nDetected chunked FFN+PREFILL model (<span class=\&quot;hljs-subst\&quot;>{total_chunks}</span> chunks)&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-comment\&quot;># Find and load all chunks</span>&quot;,&quot;            chunk_paths = find_all_chunks(ffn_path)&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-built_in\&quot;>len</span>(chunk_paths) != total_chunks:&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>raise</span> ValueError(<span class=\&quot;hljs-string\&quot;>f&amp;quot;Found <span class=\&quot;hljs-subst\&quot;>{<span class=\&quot;hljs-built_in\&quot;>len</span>(chunk_paths)}</span> chunks but filename indicates <span class=\&quot;hljs-subst\&quot;>{total_chunks}</span> chunks&amp;quot;</span>)&quot;,&quot;                &quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>for</span> chunk_path <span class=\&quot;hljs-keyword\&quot;>in</span> chunk_paths:&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;                    <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nLoading FFN+PREFILL chunk: <span class=\&quot;hljs-subst\&quot;>{Path(chunk_path).name}</span>&amp;quot;</span>)&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>try</span>:&quot;,&quot;                    <span class=\&quot;hljs-comment\&quot;># For chunked models, we need both infer and prefill functions</span>&quot;,&quot;                    ffn_models.append({&quot;,&quot;                        <span class=\&quot;hljs-string\&quot;>&amp;#x27;infer&amp;#x27;</span>: load_model(chunk_path, function_name=<span class=\&quot;hljs-string\&quot;>&amp;#x27;infer&amp;#x27;</span>),&quot;,&quot;                        <span class=\&quot;hljs-string\&quot;>&amp;#x27;prefill&amp;#x27;</span>: load_model(chunk_path, function_name=<span class=\&quot;hljs-string\&quot;>&amp;#x27;prefill&amp;#x27;</span>)&quot;,&quot;                    })&quot;,&quot;                    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;                        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;Chunk loaded successfully&amp;quot;</span>)&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>except</span> Exception <span class=\&quot;hljs-keyword\&quot;>as</span> e:&quot;,&quot;                    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;                        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;Error loading chunk <span class=\&quot;hljs-subst\&quot;>{chunk_path}</span>: <span class=\&quot;hljs-subst\&quot;>{<span class=\&quot;hljs-built_in\&quot;>str</span>(e)}</span>&amp;quot;</span>)&quot;,&quot;                    <span class=\&quot;hljs-keyword\&quot;>raise</span>&quot;,&quot;            metadata = load_metadata(ffn_models[<span class=\&quot;hljs-number\&quot;>0</span>],args)&quot;,&quot;&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nLoading single FFN model...&amp;quot;</span>)&quot;,&quot;            ffn_models.append(load_model(ffn_path))&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;FFN model loaded successfully&amp;quot;</span>)&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>return</span> embed_model, ffn_models, lmhead_model, metadata&quot;,&quot;        &quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>except</span> Exception <span class=\&quot;hljs-keyword\&quot;>as</span> e:&quot;,&quot;        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nError loading models: <span class=\&quot;hljs-subst\&quot;>{<span class=\&quot;hljs-built_in\&quot;>str</span>(e)}</span>&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nPlease ensure all model files exist and are accessible.&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;Expected files:&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  Embeddings: <span class=\&quot;hljs-subst\&quot;>{args.embed}</span>&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  LM Head: <span class=\&quot;hljs-subst\&quot;>{args.lmhead}</span>&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  FFN: <span class=\&quot;hljs-subst\&quot;>{args.ffn}</span>&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>raise</span>&quot;,&quot;&quot;,&quot;<span class=\&quot;hljs-comment\&quot;># At the top of the file, make this a default path</span>&quot;,&quot;&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>initialize_tokenizer</span>(<span class=\&quot;hljs-params\&quot;>model_path=<span class=\&quot;hljs-literal\&quot;>None</span>, eval_mode=<span class=\&quot;hljs-literal\&quot;>False</span></span>):&quot;,&quot;    <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;quot;&amp;quot;Initialize and configure the tokenizer.&amp;quot;&amp;quot;&amp;quot;</span>&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>try</span>:&quot;,&quot;&quot;,&quot;        &quot;,&quot;        tokenizer = AutoTokenizer.from_pretrained(&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>str</span>(model_path), &quot;,&quot;            use_fast=<span class=\&quot;hljs-literal\&quot;>False</span>,&quot;,&quot;            trust_remote_code=<span class=\&quot;hljs-literal\&quot;>True</span>&quot;,&quot;        )&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> eval_mode:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nTokenizer Configuration:&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;Tokenizer type: <span class=\&quot;hljs-subst\&quot;>{<span class=\&quot;hljs-built_in\&quot;>type</span>(tokenizer)}</span>&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;Tokenizer name: <span class=\&quot;hljs-subst\&quot;>{tokenizer.__class__.__name__}</span>&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;Vocabulary size: <span class=\&quot;hljs-subst\&quot;>{<span class=\&quot;hljs-built_in\&quot;>len</span>(tokenizer)}</span>&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;Model max length: <span class=\&quot;hljs-subst\&quot;>{tokenizer.model_max_length}</span>&amp;quot;</span>)&quot;,&quot;&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> tokenizer.pad_token <span class=\&quot;hljs-keyword\&quot;>is</span> <span class=\&quot;hljs-literal\&quot;>None</span>:&quot;,&quot;            tokenizer.pad_token = tokenizer.eos_token&quot;,&quot;            tokenizer.pad_token_id = tokenizer.eos_token_id&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> eval_mode:&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;Set PAD token to EOS token&amp;quot;</span>)&quot;,&quot;        &quot;,&quot;        tokenizer.padding_side = <span class=\&quot;hljs-string\&quot;>&amp;quot;left&amp;quot;</span>&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> eval_mode:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nSpecial Tokens:&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;PAD token: &amp;#x27;<span class=\&quot;hljs-subst\&quot;>{tokenizer.pad_token}</span>&amp;#x27; (ID: <span class=\&quot;hljs-subst\&quot;>{tokenizer.pad_token_id}</span>)&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;EOS token: &amp;#x27;<span class=\&quot;hljs-subst\&quot;>{tokenizer.eos_token}</span>&amp;#x27; (ID: <span class=\&quot;hljs-subst\&quot;>{tokenizer.eos_token_id}</span>)&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;BOS token: &amp;#x27;<span class=\&quot;hljs-subst\&quot;>{tokenizer.bos_token}</span>&amp;#x27; (ID: <span class=\&quot;hljs-subst\&quot;>{tokenizer.bos_token_id}</span>)&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;UNK token: &amp;#x27;<span class=\&quot;hljs-subst\&quot;>{tokenizer.unk_token}</span>&amp;#x27; (ID: <span class=\&quot;hljs-subst\&quot;>{tokenizer.unk_token_id}</span>)&amp;quot;</span>)&quot;,&quot;&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>return</span> tokenizer&quot;,&quot;        &quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>except</span> Exception <span class=\&quot;hljs-keyword\&quot;>as</span> e:&quot;,&quot;        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nError: Failed to load tokenizer from <span class=\&quot;hljs-subst\&quot;>{model_path}</span>&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;Error details: <span class=\&quot;hljs-subst\&quot;>{<span class=\&quot;hljs-built_in\&quot;>str</span>(e)}</span>&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;Error type: <span class=\&quot;hljs-subst\&quot;>{<span class=\&quot;hljs-built_in\&quot;>type</span>(e)}</span>&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nThis appears to be a tokenizer loading issue.&amp;quot;</span>)&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Check if it&amp;#x27;s the specific Qwen tokenizer file issue</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-string\&quot;>&amp;quot;expected str, bytes or os.PathLike object, not NoneType&amp;quot;</span> <span class=\&quot;hljs-keyword\&quot;>in</span> <span class=\&quot;hljs-built_in\&quot;>str</span>(e):&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nThis error suggests the tokenizer files are missing or incomplete.&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;For Qwen models, you need the original model directory with tokenizer files.&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;Try using: --tokenizer ~/.cache/huggingface/hub/models--Qwen--Qwen3-0.6B/snapshots/YOUR_SNAPSHOT_ID&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;Please provide the path to a compatible model directory with tokenizer files.&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>import</span> traceback&quot;,&quot;        traceback.print_exc()&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>raise</span>&quot;,&quot;&quot;,&quot;&quot;,&quot;&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>make_causal_mask</span>(<span class=\&quot;hljs-params\&quot;>length, start</span>):&quot;,&quot;    <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;quot;&amp;quot;Create causal attention mask.&amp;quot;&amp;quot;&amp;quot;</span>&quot;,&quot;    mask = np.full((<span class=\&quot;hljs-number\&quot;>1</span>, <span class=\&quot;hljs-number\&quot;>1</span>, length, length), -np.inf, dtype=np.float16)&quot;,&quot;    row_indices = np.arange(length).reshape(length, <span class=\&quot;hljs-number\&quot;>1</span>)&quot;,&quot;    col_indices = np.arange(length).reshape(<span class=\&quot;hljs-number\&quot;>1</span>, length)&quot;,&quot;    mask[:, :, col_indices &amp;lt;= (row_indices + start)] = <span class=\&quot;hljs-number\&quot;>0</span>&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>return</span> mask&quot;,&quot;&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>initialize_causal_mask</span>(<span class=\&quot;hljs-params\&quot;>context_length, eval_mode=<span class=\&quot;hljs-literal\&quot;>False</span></span>):&quot;,&quot;    <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;quot;&amp;quot;Initialize causal mask for transformer attention.&amp;quot;&amp;quot;&amp;quot;</span>&quot;,&quot;    causal_mask = make_causal_mask(context_length, <span class=\&quot;hljs-number\&quot;>0</span>)&quot;,&quot;    causal_mask = torch.tensor(causal_mask, dtype=torch.float16)&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> eval_mode:&quot;,&quot;        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nInitialized causal mask for context length <span class=\&quot;hljs-subst\&quot;>{context_length}</span>&amp;quot;</span>)&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>return</span> causal_mask&quot;,&quot;&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>run_prefill</span>(<span class=\&quot;hljs-params\&quot;>embed_model, ffn_models, input_ids, context_pos, context_length, batch_size=<span class=\&quot;hljs-number\&quot;>64</span>, state=<span class=\&quot;hljs-literal\&quot;>None</span>, causal_mask=<span class=\&quot;hljs-literal\&quot;>None</span></span>):&quot;,&quot;    <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;quot;&amp;quot;Run prefill on the input sequence.&amp;quot;&amp;quot;&amp;quot;</span>&quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Use provided causal mask or create one if not provided</span>&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>if</span> causal_mask <span class=\&quot;hljs-keyword\&quot;>is</span> <span class=\&quot;hljs-literal\&quot;>None</span>:&quot;,&quot;        causal_mask = make_causal_mask(context_length, <span class=\&quot;hljs-number\&quot;>0</span>)&quot;,&quot;        causal_mask = torch.tensor(causal_mask, dtype=torch.float16)&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Process in batches</span>&quot;,&quot;    batch_pos = <span class=\&quot;hljs-number\&quot;>0</span>&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>while</span> batch_pos &amp;lt; context_pos:&quot;,&quot;        batch_end = <span class=\&quot;hljs-built_in\&quot;>min</span>(batch_pos + batch_size, context_pos)&quot;,&quot;        current_batch_size = batch_end - batch_pos&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Get current batch</span>&quot;,&quot;        batch_input = input_ids[:, batch_pos:batch_end]&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Always pad to full batch size for prefill</span>&quot;,&quot;        batch_input = F.pad(&quot;,&quot;            batch_input,&quot;,&quot;            (<span class=\&quot;hljs-number\&quot;>0</span>, batch_size - current_batch_size),&quot;,&quot;            value=<span class=\&quot;hljs-number\&quot;>0</span>&quot;,&quot;        )&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Generate position IDs for full batch size</span>&quot;,&quot;        position_ids = torch.arange(batch_pos, batch_pos+batch_size, dtype=torch.int32)  <span class=\&quot;hljs-comment\&quot;># Changed: Always use full batch size</span>&quot;,&quot;        batch_causal_mask = causal_mask[:, :, batch_pos:batch_pos+batch_size, :]  <span class=\&quot;hljs-comment\&quot;># Changed: Use full batch size</span>&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Run embeddings</span>&quot;,&quot;        hidden_states = torch.from_numpy(&quot;,&quot;            embed_model.predict({&quot;,&quot;                <span class=\&quot;hljs-string\&quot;>&amp;#x27;input_ids&amp;#x27;</span>: batch_input.numpy().astype(np.int32)&quot;,&quot;            })[<span class=\&quot;hljs-string\&quot;>&amp;#x27;hidden_states&amp;#x27;</span>]&quot;,&quot;        )&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Run through FFN chunks with state</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>for</span> ffn_model <span class=\&quot;hljs-keyword\&quot;>in</span> ffn_models:&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-built_in\&quot;>isinstance</span>(ffn_model, <span class=\&quot;hljs-built_in\&quot;>dict</span>):&quot;,&quot;                inputs = {&quot;,&quot;                    <span class=\&quot;hljs-string\&quot;>&amp;#x27;hidden_states&amp;#x27;</span>: hidden_states.numpy().astype(np.float16),  <span class=\&quot;hljs-comment\&quot;># [1, 64, hidden_size]</span>&quot;,&quot;                    <span class=\&quot;hljs-string\&quot;>&amp;#x27;position_ids&amp;#x27;</span>: position_ids.numpy().astype(np.int32),    <span class=\&quot;hljs-comment\&quot;># [64]</span>&quot;,&quot;                    <span class=\&quot;hljs-string\&quot;>&amp;#x27;causal_mask&amp;#x27;</span>: batch_causal_mask.numpy().astype(np.float16), <span class=\&quot;hljs-comment\&quot;># [1, 1, 64, context_length]</span>&quot;,&quot;                    <span class=\&quot;hljs-string\&quot;>&amp;#x27;current_pos&amp;#x27;</span>: np.array([batch_pos], dtype=np.int32)  <span class=\&quot;hljs-comment\&quot;># [1]</span>&quot;,&quot;                }&quot;,&quot;                output = ffn_model[<span class=\&quot;hljs-string\&quot;>&amp;#x27;prefill&amp;#x27;</span>].predict(inputs, state)&quot;,&quot;                hidden_states = torch.from_numpy(output[<span class=\&quot;hljs-string\&quot;>&amp;#x27;output_hidden_states&amp;#x27;</span>])&quot;,&quot;        &quot;,&quot;        batch_pos = batch_end&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>return</span> torch.tensor([context_pos], dtype=torch.int32)&quot;,&quot;&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>generate_next_token</span>(<span class=\&quot;hljs-params\&quot;>embed_model, ffn_models, lmhead_model, input_ids, pos, context_length, metadata, state=<span class=\&quot;hljs-literal\&quot;>None</span>, causal_mask=<span class=\&quot;hljs-literal\&quot;>None</span>, temperature=<span class=\&quot;hljs-number\&quot;>0.0</span></span>):&quot;,&quot;    <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;quot;&amp;quot;Generate the next token.&amp;quot;&amp;quot;&amp;quot;</span>&quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Get current token</span>&quot;,&quot;    current_token = input_ids[:, pos-<span class=\&quot;hljs-number\&quot;>1</span>:pos]  <span class=\&quot;hljs-comment\&quot;># [1, 1]</span>&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Ensure proper data type for CoreML</span>&quot;,&quot;    current_token_array = current_token.numpy().astype(np.int32)&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Run embeddings</span>&quot;,&quot;    hidden_states = torch.from_numpy(&quot;,&quot;        embed_model.predict({<span class=\&quot;hljs-string\&quot;>&amp;#x27;input_ids&amp;#x27;</span>: current_token_array})[<span class=\&quot;hljs-string\&quot;>&amp;#x27;hidden_states&amp;#x27;</span>]&quot;,&quot;    )  <span class=\&quot;hljs-comment\&quot;># [1, 1, hidden_size]</span>&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Create masks</span>&quot;,&quot;    update_mask = torch.zeros((<span class=\&quot;hljs-number\&quot;>1</span>, <span class=\&quot;hljs-number\&quot;>1</span>, context_length, <span class=\&quot;hljs-number\&quot;>1</span>), dtype=torch.float16)&quot;,&quot;    update_mask[<span class=\&quot;hljs-number\&quot;>0</span>, <span class=\&quot;hljs-number\&quot;>0</span>, pos-<span class=\&quot;hljs-number\&quot;>1</span>, <span class=\&quot;hljs-number\&quot;>0</span>] = <span class=\&quot;hljs-number\&quot;>1.0</span>&quot;,&quot;    position_ids = torch.tensor([pos-<span class=\&quot;hljs-number\&quot;>1</span>], dtype=torch.int32)  <span class=\&quot;hljs-comment\&quot;># [1]</span>&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Use provided causal mask or create one if not provided</span>&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>if</span> causal_mask <span class=\&quot;hljs-keyword\&quot;>is</span> <span class=\&quot;hljs-literal\&quot;>None</span>:&quot;,&quot;        causal_mask_data = make_causal_mask(context_length, <span class=\&quot;hljs-number\&quot;>0</span>)&quot;,&quot;        single_causal_mask = torch.tensor(causal_mask_data[:, :, pos-<span class=\&quot;hljs-number\&quot;>1</span>:pos, :], dtype=torch.float16)  <span class=\&quot;hljs-comment\&quot;># [1, 1, 1, context_length]</span>&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;        single_causal_mask = causal_mask[:, :, pos-<span class=\&quot;hljs-number\&quot;>1</span>:pos, :]&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Run through FFN chunks with state</span>&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>for</span> ffn_model <span class=\&quot;hljs-keyword\&quot;>in</span> ffn_models:&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-built_in\&quot;>isinstance</span>(ffn_model, <span class=\&quot;hljs-built_in\&quot;>dict</span>):&quot;,&quot;            inputs = {&quot;,&quot;                <span class=\&quot;hljs-string\&quot;>&amp;#x27;hidden_states&amp;#x27;</span>: hidden_states.numpy().astype(np.float16),&quot;,&quot;                <span class=\&quot;hljs-string\&quot;>&amp;#x27;update_mask&amp;#x27;</span>: update_mask.numpy().astype(np.float16),&quot;,&quot;                <span class=\&quot;hljs-string\&quot;>&amp;#x27;position_ids&amp;#x27;</span>: position_ids.numpy().astype(np.int32),&quot;,&quot;                <span class=\&quot;hljs-string\&quot;>&amp;#x27;causal_mask&amp;#x27;</span>: single_causal_mask.numpy().astype(np.float16),&quot;,&quot;                <span class=\&quot;hljs-string\&quot;>&amp;#x27;current_pos&amp;#x27;</span>: position_ids.numpy().astype(np.int32)&quot;,&quot;            }&quot;,&quot;            output = ffn_model[<span class=\&quot;hljs-string\&quot;>&amp;#x27;infer&amp;#x27;</span>].predict(inputs, state)&quot;,&quot;            hidden_states = torch.from_numpy(output[<span class=\&quot;hljs-string\&quot;>&amp;#x27;output_hidden_states&amp;#x27;</span>])&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Run LM head</span>&quot;,&quot;    lm_output = lmhead_model.predict({<span class=\&quot;hljs-string\&quot;>&amp;#x27;hidden_states&amp;#x27;</span>: hidden_states.numpy().astype(np.float16)})&quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Debug print</span>&quot;,&quot;    <span class=\&quot;hljs-comment\&quot;>#print(&amp;quot;\\nLM Head output keys:&amp;quot;, list(lm_output.keys()))</span>&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Get number of logits from metadata, using split_lm_head if available</span>&quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># First check for split_lm_head (new), then num_logits (legacy), default to 8</span>&quot;,&quot;    num_logits = metadata.get(<span class=\&quot;hljs-string\&quot;>&amp;#x27;split_lm_head&amp;#x27;</span>, metadata.get(<span class=\&quot;hljs-string\&quot;>&amp;#x27;num_logits&amp;#x27;</span>, <span class=\&quot;hljs-number\&quot;>8</span>))&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Combine logits1-N if they exist</span>&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-string\&quot;>&amp;#x27;logits1&amp;#x27;</span> <span class=\&quot;hljs-keyword\&quot;>in</span> lm_output:&quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Concatenate all logits parts</span>&quot;,&quot;        logits_parts = []&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>for</span> i <span class=\&quot;hljs-keyword\&quot;>in</span> <span class=\&quot;hljs-built_in\&quot;>range</span>(<span class=\&quot;hljs-number\&quot;>1</span>, num_logits + <span class=\&quot;hljs-number\&quot;>1</span>):&quot;,&quot;            key = <span class=\&quot;hljs-string\&quot;>f&amp;#x27;logits<span class=\&quot;hljs-subst\&quot;>{i}</span>&amp;#x27;</span>&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> key <span class=\&quot;hljs-keyword\&quot;>in</span> lm_output:&quot;,&quot;                logits_parts.append(torch.from_numpy(lm_output[key]))&quot;,&quot;        logits = torch.cat(logits_parts, dim=-<span class=\&quot;hljs-number\&quot;>1</span>)  <span class=\&quot;hljs-comment\&quot;># Concatenate along vocab dimension</span>&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Try output_logits as fallback</span>&quot;,&quot;        logits = torch.from_numpy(lm_output[<span class=\&quot;hljs-string\&quot;>&amp;#x27;output_logits&amp;#x27;</span>])&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Apply temperature and sample</span>&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>if</span> temperature &amp;gt; <span class=\&quot;hljs-number\&quot;>0</span>:&quot;,&quot;        logits = logits / temperature&quot;,&quot;        probs = F.softmax(logits[<span class=\&quot;hljs-number\&quot;>0</span>, -<span class=\&quot;hljs-number\&quot;>1</span>, :], dim=-<span class=\&quot;hljs-number\&quot;>1</span>)&quot;,&quot;        next_token = torch.multinomial(probs, num_samples=<span class=\&quot;hljs-number\&quot;>1</span>).item()&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;        next_token = torch.argmax(logits[<span class=\&quot;hljs-number\&quot;>0</span>, -<span class=\&quot;hljs-number\&quot;>1</span>, :]).item()&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>return</span> next_token&quot;,&quot;&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>create_unified_state</span>(<span class=\&quot;hljs-params\&quot;>ffn_models, context_length, eval_mode=<span class=\&quot;hljs-literal\&quot;>False</span></span>):&quot;,&quot;    <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;quot;&amp;quot;Create unified KV cache state for transformer.&amp;quot;&amp;quot;&amp;quot;</span>&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-built_in\&quot;>isinstance</span>(ffn_models[<span class=\&quot;hljs-number\&quot;>0</span>], <span class=\&quot;hljs-built_in\&quot;>dict</span>):&quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Use first FFN model&amp;#x27;s prefill function to create state</span>&quot;,&quot;        state = ffn_models[<span class=\&quot;hljs-number\&quot;>0</span>][<span class=\&quot;hljs-string\&quot;>&amp;#x27;prefill&amp;#x27;</span>].make_state()&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> eval_mode:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nCreated unified transformer state for <span class=\&quot;hljs-subst\&quot;>{<span class=\&quot;hljs-built_in\&quot;>len</span>(ffn_models)}</span> chunks&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>return</span> state&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;        state = ffn_models[<span class=\&quot;hljs-number\&quot;>0</span>].make_state()&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> eval_mode:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nCreated unified transformer state&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>return</span> state&quot;,&quot;&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>chat_loop</span>(<span class=\&quot;hljs-params\&quot;>embed_model, ffn_models, lmhead_model, tokenizer, metadata, state, causal_mask=<span class=\&quot;hljs-literal\&quot;>None</span>, auto_prompt=<span class=\&quot;hljs-literal\&quot;>None</span>, warmup=<span class=\&quot;hljs-literal\&quot;>False</span>, save_file=<span class=\&quot;hljs-literal\&quot;>None</span>, max_tokens=<span class=\&quot;hljs-literal\&quot;>None</span>, no_template=<span class=\&quot;hljs-literal\&quot;>False</span>, eval_mode=<span class=\&quot;hljs-literal\&quot;>False</span></span>):&quot;,&quot;    <span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;quot;&amp;quot;Interactive chat loop.&amp;quot;&amp;quot;&amp;quot;</span>&quot;,&quot;    context_length = metadata.get(<span class=\&quot;hljs-string\&quot;>&amp;#x27;context_length&amp;#x27;</span>)&quot;,&quot;    batch_size = metadata.get(<span class=\&quot;hljs-string\&quot;>&amp;#x27;batch_size&amp;#x27;</span>, <span class=\&quot;hljs-number\&quot;>64</span>)&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> warmup <span class=\&quot;hljs-keyword\&quot;>and</span> <span class=\&quot;hljs-keyword\&quot;>not</span> eval_mode:&quot;,&quot;        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nUsing context length: <span class=\&quot;hljs-subst\&quot;>{context_length}</span>&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nStarting chat session. Press Ctrl+D to exit.&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;Type your message and press Enter to chat.&amp;quot;</span>)&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Check if tokenizer has chat template and if it works</span>&quot;,&quot;    has_chat_template = <span class=\&quot;hljs-literal\&quot;>False</span>&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>try</span>:&quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Test if chat template works</span>&quot;,&quot;        test_messages = [{<span class=\&quot;hljs-string\&quot;>&amp;quot;role&amp;quot;</span>: <span class=\&quot;hljs-string\&quot;>&amp;quot;user&amp;quot;</span>, <span class=\&quot;hljs-string\&quot;>&amp;quot;content&amp;quot;</span>: <span class=\&quot;hljs-string\&quot;>&amp;quot;test&amp;quot;</span>}]&quot;,&quot;        tokenizer.apply_chat_template(test_messages, return_tensors=<span class=\&quot;hljs-string\&quot;>&amp;quot;pt&amp;quot;</span>)&quot;,&quot;        has_chat_template = <span class=\&quot;hljs-literal\&quot;>True</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> warmup <span class=\&quot;hljs-keyword\&quot;>and</span> <span class=\&quot;hljs-keyword\&quot;>not</span> eval_mode:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nUsing chat template for prompts&amp;quot;</span>)&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>except</span>:&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> warmup <span class=\&quot;hljs-keyword\&quot;>and</span> <span class=\&quot;hljs-keyword\&quot;>not</span> eval_mode:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nUsing manual formatting for prompts&amp;quot;</span>)&quot;,&quot;    &quot;,&quot;    conversation = []&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>try</span>:&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>while</span> <span class=\&quot;hljs-literal\&quot;>True</span>:&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>try</span>:&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> warmup <span class=\&quot;hljs-keyword\&quot;>and</span> <span class=\&quot;hljs-keyword\&quot;>not</span> eval_mode:&quot;,&quot;                    <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\n<span class=\&quot;hljs-subst\&quot;>{LIGHT_GREEN}</span>You:<span class=\&quot;hljs-subst\&quot;>{RESET_COLOR}</span>&amp;quot;</span>, end=<span class=\&quot;hljs-string\&quot;>&amp;#x27; &amp;#x27;</span>, flush=<span class=\&quot;hljs-literal\&quot;>True</span>)&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>if</span> auto_prompt <span class=\&quot;hljs-keyword\&quot;>is</span> <span class=\&quot;hljs-keyword\&quot;>not</span> <span class=\&quot;hljs-literal\&quot;>None</span>:&quot;,&quot;                    user_input = auto_prompt&quot;,&quot;                    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> warmup <span class=\&quot;hljs-keyword\&quot;>and</span> <span class=\&quot;hljs-keyword\&quot;>not</span> eval_mode:&quot;,&quot;                        <span class=\&quot;hljs-built_in\&quot;>print</span>(user_input)&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;                    user_input = <span class=\&quot;hljs-built_in\&quot;>input</span>().strip()&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>except</span> EOFError:&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> warmup <span class=\&quot;hljs-keyword\&quot;>and</span> <span class=\&quot;hljs-keyword\&quot;>not</span> eval_mode:&quot;,&quot;                    <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nExiting chat...&amp;quot;</span>)&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>break</span>&quot;,&quot;                &quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> user_input:&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>continue</span>&quot;,&quot;            &quot;,&quot;            <span class=\&quot;hljs-comment\&quot;># Format prompt based on no_template flag and tokenizer capabilities</span>&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> no_template:&quot;,&quot;                <span class=\&quot;hljs-comment\&quot;># Use raw input without any chat template formatting</span>&quot;,&quot;                input_ids = tokenizer(&quot;,&quot;                    user_input,&quot;,&quot;                    return_tensors=<span class=\&quot;hljs-string\&quot;>&amp;quot;pt&amp;quot;</span>,&quot;,&quot;                    add_special_tokens=<span class=\&quot;hljs-literal\&quot;>True</span>&quot;,&quot;                ).input_ids.to(torch.int32)&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> warmup <span class=\&quot;hljs-keyword\&quot;>and</span> <span class=\&quot;hljs-keyword\&quot;>not</span> eval_mode:&quot;,&quot;                    <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;Using raw input without chat template&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>elif</span> has_chat_template:&quot;,&quot;                messages = [{<span class=\&quot;hljs-string\&quot;>&amp;quot;role&amp;quot;</span>: <span class=\&quot;hljs-string\&quot;>&amp;quot;user&amp;quot;</span>, <span class=\&quot;hljs-string\&quot;>&amp;quot;content&amp;quot;</span>: user_input}]&quot;,&quot;                input_ids = tokenizer.apply_chat_template(&quot;,&quot;                    messages,&quot;,&quot;                    return_tensors=<span class=\&quot;hljs-string\&quot;>&amp;quot;pt&amp;quot;</span>,&quot;,&quot;                    add_generation_prompt=<span class=\&quot;hljs-literal\&quot;>True</span>&quot;,&quot;                ).to(torch.int32)&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;                <span class=\&quot;hljs-comment\&quot;># Manual formatting for Llama models without chat template</span>&quot;,&quot;                formatted_prompt = <span class=\&quot;hljs-string\&quot;>f&amp;quot;[INST] <span class=\&quot;hljs-subst\&quot;>{user_input}</span> [/INST]&amp;quot;</span>&quot;,&quot;                input_ids = tokenizer(&quot;,&quot;                    formatted_prompt,&quot;,&quot;                    return_tensors=<span class=\&quot;hljs-string\&quot;>&amp;quot;pt&amp;quot;</span>,&quot;,&quot;                    add_special_tokens=<span class=\&quot;hljs-literal\&quot;>True</span>&quot;,&quot;                ).input_ids.to(torch.int32)&quot;,&quot;            &quot;,&quot;            context_pos = input_ids.size(<span class=\&quot;hljs-number\&quot;>1</span>)&quot;,&quot;            &quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> warmup <span class=\&quot;hljs-keyword\&quot;>and</span> <span class=\&quot;hljs-keyword\&quot;>not</span> eval_mode:&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\n<span class=\&quot;hljs-subst\&quot;>{LIGHT_BLUE}</span>Assistant:<span class=\&quot;hljs-subst\&quot;>{RESET_COLOR}</span>&amp;quot;</span>, end=<span class=\&quot;hljs-string\&quot;>&amp;#x27; &amp;#x27;</span>, flush=<span class=\&quot;hljs-literal\&quot;>True</span>)&quot;,&quot;            &quot;,&quot;            <span class=\&quot;hljs-comment\&quot;># Initialize token printer</span>&quot;,&quot;            token_printer = TokenPrinter(tokenizer)&quot;,&quot;            tokens_generated = <span class=\&quot;hljs-number\&quot;>0</span>  <span class=\&quot;hljs-comment\&quot;># Track number of tokens</span>&quot;,&quot;            &quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>try</span>:&quot;,&quot;                <span class=\&quot;hljs-comment\&quot;># Start prefill timing</span>&quot;,&quot;                prefill_start = time.time()&quot;,&quot;                &quot;,&quot;                <span class=\&quot;hljs-comment\&quot;># Run prefill with state and causal mask</span>&quot;,&quot;                <span class=\&quot;hljs-comment\&quot;># Ensure batch_size is not None</span>&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>if</span> batch_size <span class=\&quot;hljs-keyword\&quot;>is</span> <span class=\&quot;hljs-literal\&quot;>None</span>:&quot;,&quot;                    batch_size = <span class=\&quot;hljs-number\&quot;>64</span>&quot;,&quot;                    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> eval_mode:&quot;,&quot;                        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;Warning: batch_size was None, using default: <span class=\&quot;hljs-subst\&quot;>{batch_size}</span>&amp;quot;</span>)&quot;,&quot;                &quot;,&quot;                _ = run_prefill(&quot;,&quot;                    embed_model,&quot;,&quot;                    ffn_models,&quot;,&quot;                    input_ids,&quot;,&quot;                    context_pos,&quot;,&quot;                    context_length,&quot;,&quot;                    batch_size,&quot;,&quot;                    state,&quot;,&quot;                    causal_mask&quot;,&quot;                )&quot;,&quot;                &quot;,&quot;                <span class=\&quot;hljs-comment\&quot;># Calculate prefill timing</span>&quot;,&quot;                prefill_time = time.time() - prefill_start&quot;,&quot;                prefill_tokens = context_pos  <span class=\&quot;hljs-comment\&quot;># Number of tokens in input</span>&quot;,&quot;                prefill_tokens_per_sec = prefill_tokens / prefill_time <span class=\&quot;hljs-keyword\&quot;>if</span> prefill_time &amp;gt; <span class=\&quot;hljs-number\&quot;>0</span> <span class=\&quot;hljs-keyword\&quot;>else</span> <span class=\&quot;hljs-number\&quot;>0</span>&quot;,&quot;                &quot;,&quot;                <span class=\&quot;hljs-comment\&quot;># Generation loop with state</span>&quot;,&quot;                input_ids = input_ids&quot;,&quot;                pos = context_pos&quot;,&quot;                inference_start = time.time()&quot;,&quot;                inference_tokens = <span class=\&quot;hljs-number\&quot;>0</span>&quot;,&quot;                &quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>while</span> pos &amp;lt; context_length - <span class=\&quot;hljs-number\&quot;>1</span>:&quot;,&quot;                    <span class=\&quot;hljs-comment\&quot;># Generate next token with causal mask</span>&quot;,&quot;                    next_token = generate_next_token(&quot;,&quot;                        embed_model,&quot;,&quot;                        ffn_models,&quot;,&quot;                        lmhead_model,&quot;,&quot;                        input_ids,&quot;,&quot;                        pos,&quot;,&quot;                        context_length,&quot;,&quot;                        metadata,&quot;,&quot;                        state,&quot;,&quot;                        causal_mask&quot;,&quot;                    )&quot;,&quot;                    &quot;,&quot;                    <span class=\&quot;hljs-comment\&quot;># Add token to sequence</span>&quot;,&quot;                    <span class=\&quot;hljs-keyword\&quot;>if</span> pos &amp;lt; input_ids.size(<span class=\&quot;hljs-number\&quot;>1</span>):&quot;,&quot;                        input_ids[<span class=\&quot;hljs-number\&quot;>0</span>, pos] = next_token&quot;,&quot;                    <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;                        input_ids = torch.cat([&quot;,&quot;                            input_ids,&quot;,&quot;                            torch.tensor([[next_token]], dtype=torch.int32)&quot;,&quot;                        ], dim=<span class=\&quot;hljs-number\&quot;>1</span>)&quot;,&quot;                    &quot;,&quot;                    <span class=\&quot;hljs-comment\&quot;># Add to printer only if not in warmup</span>&quot;,&quot;                    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> warmup:&quot;,&quot;                        token_printer.add_token(next_token)&quot;,&quot;                        token_printer.drain_buffer(eval_mode)&quot;,&quot;                    &quot;,&quot;                    pos += <span class=\&quot;hljs-number\&quot;>1</span>&quot;,&quot;                    tokens_generated += <span class=\&quot;hljs-number\&quot;>1</span>&quot;,&quot;                    inference_tokens += <span class=\&quot;hljs-number\&quot;>1</span>&quot;,&quot;                    &quot;,&quot;                    <span class=\&quot;hljs-comment\&quot;># Check limits</span>&quot;,&quot;                    <span class=\&quot;hljs-keyword\&quot;>if</span> warmup <span class=\&quot;hljs-keyword\&quot;>and</span> tokens_generated &amp;gt;= WARMUP_TOKEN_LIMIT:&quot;,&quot;                        <span class=\&quot;hljs-keyword\&quot;>break</span>&quot;,&quot;                    &quot;,&quot;                    <span class=\&quot;hljs-comment\&quot;># Check max_tokens limit</span>&quot;,&quot;                    <span class=\&quot;hljs-keyword\&quot;>if</span> max_tokens <span class=\&quot;hljs-keyword\&quot;>is</span> <span class=\&quot;hljs-keyword\&quot;>not</span> <span class=\&quot;hljs-literal\&quot;>None</span> <span class=\&quot;hljs-keyword\&quot;>and</span> tokens_generated &amp;gt;= max_tokens:&quot;,&quot;                        <span class=\&quot;hljs-keyword\&quot;>break</span>&quot;,&quot;                        &quot;,&quot;                    <span class=\&quot;hljs-comment\&quot;># Check for all possible EOS tokens</span>&quot;,&quot;                    eos_token_ids = tokenizer.eos_token_id&quot;,&quot;                    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-built_in\&quot;>isinstance</span>(eos_token_ids, <span class=\&quot;hljs-built_in\&quot;>list</span>):&quot;,&quot;                        <span class=\&quot;hljs-keyword\&quot;>if</span> next_token <span class=\&quot;hljs-keyword\&quot;>in</span> eos_token_ids:&quot;,&quot;                            <span class=\&quot;hljs-keyword\&quot;>break</span>&quot;,&quot;                    <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;                        <span class=\&quot;hljs-keyword\&quot;>if</span> next_token == eos_token_ids:&quot;,&quot;                            <span class=\&quot;hljs-keyword\&quot;>break</span>&quot;,&quot;                &quot;,&quot;                <span class=\&quot;hljs-comment\&quot;># Calculate inference timing</span>&quot;,&quot;                inference_time = time.time() - inference_start&quot;,&quot;                inference_tokens_per_sec = inference_tokens / inference_time <span class=\&quot;hljs-keyword\&quot;>if</span> inference_time &amp;gt; <span class=\&quot;hljs-number\&quot;>0</span> <span class=\&quot;hljs-keyword\&quot;>else</span> <span class=\&quot;hljs-number\&quot;>0</span>&quot;,&quot;                &quot;,&quot;                <span class=\&quot;hljs-comment\&quot;># Get final response and add to conversation</span>&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> warmup:&quot;,&quot;                    response = token_printer.stop(eval_mode)&quot;,&quot;                    <span class=\&quot;hljs-keyword\&quot;>if</span> eval_mode:&quot;,&quot;                        <span class=\&quot;hljs-comment\&quot;># In eval mode, only print the model response</span>&quot;,&quot;                        <span class=\&quot;hljs-built_in\&quot;>print</span>(response, end=<span class=\&quot;hljs-string\&quot;>&amp;#x27;&amp;#x27;</span>)&quot;,&quot;                    <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;                        <span class=\&quot;hljs-comment\&quot;># Print timing stats</span>&quot;,&quot;                        prefill_ms = prefill_time * <span class=\&quot;hljs-number\&quot;>1000</span>  <span class=\&quot;hljs-comment\&quot;># Convert to milliseconds</span>&quot;,&quot;                        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nPrefill: <span class=\&quot;hljs-subst\&quot;>{prefill_ms:<span class=\&quot;hljs-number\&quot;>.1</span>f}</span>ms (<span class=\&quot;hljs-subst\&quot;>{prefill_tokens_per_sec:<span class=\&quot;hljs-number\&quot;>.1</span>f}</span> t/s)&amp;quot;</span>)&quot;,&quot;                        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;Inference: <span class=\&quot;hljs-subst\&quot;>{inference_tokens_per_sec:<span class=\&quot;hljs-number\&quot;>.1</span>f}</span> t/s&amp;quot;</span>)&quot;,&quot;                        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;Total: Generated <span class=\&quot;hljs-subst\&quot;>{tokens_generated}</span> tokens in <span class=\&quot;hljs-subst\&quot;>{prefill_time + inference_time:<span class=\&quot;hljs-number\&quot;>.2</span>f}</span>s&amp;quot;</span>)&quot;,&quot;                    conversation.append({<span class=\&quot;hljs-string\&quot;>&amp;quot;role&amp;quot;</span>: <span class=\&quot;hljs-string\&quot;>&amp;quot;assistant&amp;quot;</span>, <span class=\&quot;hljs-string\&quot;>&amp;quot;content&amp;quot;</span>: response})&quot;,&quot;                    &quot;,&quot;                    <span class=\&quot;hljs-comment\&quot;># Save response to file if requested</span>&quot;,&quot;                    <span class=\&quot;hljs-keyword\&quot;>if</span> save_file <span class=\&quot;hljs-keyword\&quot;>and</span> <span class=\&quot;hljs-keyword\&quot;>not</span> eval_mode:&quot;,&quot;                        <span class=\&quot;hljs-keyword\&quot;>try</span>:&quot;,&quot;                            <span class=\&quot;hljs-comment\&quot;># Add small delay to ensure all tokens are processed</span>&quot;,&quot;                            time.sleep(<span class=\&quot;hljs-number\&quot;>0.5</span>)&quot;,&quot;                            &quot;,&quot;                            <span class=\&quot;hljs-comment\&quot;># Make sure response ends with EOS token if it&amp;#x27;s supposed to</span>&quot;,&quot;                            <span class=\&quot;hljs-keyword\&quot;>if</span> response <span class=\&quot;hljs-keyword\&quot;>and</span> <span class=\&quot;hljs-keyword\&quot;>not</span> response.endswith(<span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;lt;|eot_id|&amp;gt;&amp;quot;</span>) <span class=\&quot;hljs-keyword\&quot;>and</span> <span class=\&quot;hljs-keyword\&quot;>not</span> response.endswith(<span class=\&quot;hljs-string\&quot;>&amp;quot;&amp;lt;/s&amp;gt;&amp;quot;</span>):&quot;,&quot;                                <span class=\&quot;hljs-keyword\&quot;>if</span> tokenizer.eos_token:&quot;,&quot;                                    eos_text = tokenizer.decode([tokenizer.eos_token_id])&quot;,&quot;                                    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> response.endswith(eos_text):&quot;,&quot;                                        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\n<span class=\&quot;hljs-subst\&quot;>{DARK_BLUE}</span>Adding missing EOS token for consistency<span class=\&quot;hljs-subst\&quot;>{RESET_COLOR}</span>&amp;quot;</span>)&quot;,&quot;                                        response += eos_text&quot;,&quot;                            &quot;,&quot;                            <span class=\&quot;hljs-keyword\&quot;>with</span> <span class=\&quot;hljs-built_in\&quot;>open</span>(save_file, <span class=\&quot;hljs-string\&quot;>&amp;#x27;w&amp;#x27;</span>) <span class=\&quot;hljs-keyword\&quot;>as</span> f:&quot;,&quot;                                f.write(response)&quot;,&quot;                            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\n<span class=\&quot;hljs-subst\&quot;>{DARK_BLUE}</span>Response saved to file: <span class=\&quot;hljs-subst\&quot;>{save_file}</span><span class=\&quot;hljs-subst\&quot;>{RESET_COLOR}</span>&amp;quot;</span>)&quot;,&quot;                        <span class=\&quot;hljs-keyword\&quot;>except</span> Exception <span class=\&quot;hljs-keyword\&quot;>as</span> e:&quot;,&quot;                            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\n<span class=\&quot;hljs-subst\&quot;>{DARK_BLUE}</span>Error saving to file: <span class=\&quot;hljs-subst\&quot;>{<span class=\&quot;hljs-built_in\&quot;>str</span>(e)}</span><span class=\&quot;hljs-subst\&quot;>{RESET_COLOR}</span>&amp;quot;</span>)&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;                    token_printer.stop(eval_mode)  <span class=\&quot;hljs-comment\&quot;># Clean up without printing stats</span>&quot;,&quot;                &quot;,&quot;                <span class=\&quot;hljs-comment\&quot;># Exit after one response in auto_prompt mode</span>&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>if</span> auto_prompt <span class=\&quot;hljs-keyword\&quot;>is</span> <span class=\&quot;hljs-keyword\&quot;>not</span> <span class=\&quot;hljs-literal\&quot;>None</span>:&quot;,&quot;                    <span class=\&quot;hljs-keyword\&quot;>break</span>&quot;,&quot;                &quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>except</span> KeyboardInterrupt:&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> eval_mode:&quot;,&quot;                    <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nGeneration interrupted&amp;quot;</span>)&quot;,&quot;                token_printer.stop(eval_mode)&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>continue</span>&quot;,&quot;                &quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>except</span> Exception <span class=\&quot;hljs-keyword\&quot;>as</span> e:&quot;,&quot;        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nError in chat loop: <span class=\&quot;hljs-subst\&quot;>{<span class=\&quot;hljs-built_in\&quot;>str</span>(e)}</span>&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>import</span> traceback&quot;,&quot;        traceback.print_exc()&quot;,&quot;&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>parse_args</span>():&quot;,&quot;    parser = argparse.ArgumentParser(description=<span class=\&quot;hljs-string\&quot;>&amp;#x27;Chat with CoreML LLaMA, gil resolved  (c) 2025 Anemll&amp;#x27;</span>)&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Add meta.yaml option</span>&quot;,&quot;    parser.add_argument(<span class=\&quot;hljs-string\&quot;>&amp;#x27;--meta&amp;#x27;</span>, <span class=\&quot;hljs-built_in\&quot;>type</span>=<span class=\&quot;hljs-built_in\&quot;>str</span>, <span class=\&quot;hljs-built_in\&quot;>help</span>=<span class=\&quot;hljs-string\&quot;>&amp;#x27;Path to meta.yaml to load all parameters&amp;#x27;</span>)&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Model paths</span>&quot;,&quot;    parser.add_argument(<span class=\&quot;hljs-string\&quot;>&amp;#x27;--d&amp;#x27;</span>, <span class=\&quot;hljs-string\&quot;>&amp;#x27;--dir&amp;#x27;</span>, <span class=\&quot;hljs-built_in\&quot;>type</span>=<span class=\&quot;hljs-built_in\&quot;>str</span>, default=<span class=\&quot;hljs-string\&quot;>&amp;#x27;.&amp;#x27;</span>,&quot;,&quot;                       <span class=\&quot;hljs-built_in\&quot;>help</span>=<span class=\&quot;hljs-string\&quot;>&amp;#x27;Directory containing model files (default: current directory)&amp;#x27;</span>)&quot;,&quot;    parser.add_argument(<span class=\&quot;hljs-string\&quot;>&amp;#x27;--embed&amp;#x27;</span>, <span class=\&quot;hljs-built_in\&quot;>type</span>=<span class=\&quot;hljs-built_in\&quot;>str</span>, required=<span class=\&quot;hljs-literal\&quot;>False</span>,&quot;,&quot;                       <span class=\&quot;hljs-built_in\&quot;>help</span>=<span class=\&quot;hljs-string\&quot;>&amp;#x27;Path to embeddings model (relative to --dir)&amp;#x27;</span>)&quot;,&quot;    parser.add_argument(<span class=\&quot;hljs-string\&quot;>&amp;#x27;--ffn&amp;#x27;</span>, <span class=\&quot;hljs-built_in\&quot;>type</span>=<span class=\&quot;hljs-built_in\&quot;>str</span>, required=<span class=\&quot;hljs-literal\&quot;>False</span>,&quot;,&quot;                       <span class=\&quot;hljs-built_in\&quot;>help</span>=<span class=\&quot;hljs-string\&quot;>&amp;#x27;Path to FFN model (can be chunked, relative to --dir)&amp;#x27;</span>)&quot;,&quot;    parser.add_argument(<span class=\&quot;hljs-string\&quot;>&amp;#x27;--lmhead&amp;#x27;</span>, <span class=\&quot;hljs-built_in\&quot;>type</span>=<span class=\&quot;hljs-built_in\&quot;>str</span>, required=<span class=\&quot;hljs-literal\&quot;>False</span>,&quot;,&quot;                       <span class=\&quot;hljs-built_in\&quot;>help</span>=<span class=\&quot;hljs-string\&quot;>&amp;#x27;Path to LM head model (relative to --dir)&amp;#x27;</span>)&quot;,&quot;    parser.add_argument(<span class=\&quot;hljs-string\&quot;>&amp;#x27;--tokenizer&amp;#x27;</span>, <span class=\&quot;hljs-built_in\&quot;>type</span>=<span class=\&quot;hljs-built_in\&quot;>str</span>, required=<span class=\&quot;hljs-literal\&quot;>False</span>,&quot;,&quot;                       <span class=\&quot;hljs-built_in\&quot;>help</span>=<span class=\&quot;hljs-string\&quot;>&amp;#x27;Path to tokenizer&amp;#x27;</span>)&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Add new argument for auto-generation</span>&quot;,&quot;    parser.add_argument(<span class=\&quot;hljs-string\&quot;>&amp;#x27;--prompt&amp;#x27;</span>, <span class=\&quot;hljs-built_in\&quot;>type</span>=<span class=\&quot;hljs-built_in\&quot;>str</span>,&quot;,&quot;                       <span class=\&quot;hljs-built_in\&quot;>help</span>=<span class=\&quot;hljs-string\&quot;>&amp;#x27;If specified, run once with this prompt and exit&amp;#x27;</span>)&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Add save option</span>&quot;,&quot;    parser.add_argument(<span class=\&quot;hljs-string\&quot;>&amp;#x27;--save&amp;#x27;</span>, <span class=\&quot;hljs-built_in\&quot;>type</span>=<span class=\&quot;hljs-built_in\&quot;>str</span>,&quot;,&quot;                       <span class=\&quot;hljs-built_in\&quot;>help</span>=<span class=\&quot;hljs-string\&quot;>&amp;#x27;Save assistant\\&amp;#x27;s response to specified file&amp;#x27;</span>)&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Add max-tokens option</span>&quot;,&quot;    parser.add_argument(<span class=\&quot;hljs-string\&quot;>&amp;#x27;--max-tokens&amp;#x27;</span>, <span class=\&quot;hljs-built_in\&quot;>type</span>=<span class=\&quot;hljs-built_in\&quot;>int</span>,&quot;,&quot;                       <span class=\&quot;hljs-built_in\&quot;>help</span>=<span class=\&quot;hljs-string\&quot;>&amp;#x27;Maximum number of tokens to generate&amp;#x27;</span>)&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Add no-warmup flag</span>&quot;,&quot;    parser.add_argument(<span class=\&quot;hljs-string\&quot;>&amp;#x27;--nw&amp;#x27;</span>, action=<span class=\&quot;hljs-string\&quot;>&amp;#x27;store_true&amp;#x27;</span>,&quot;,&quot;                       <span class=\&quot;hljs-built_in\&quot;>help</span>=<span class=\&quot;hljs-string\&quot;>&amp;#x27;Skip warmup phase&amp;#x27;</span>)&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Add no-template flag  </span>&quot;,&quot;    parser.add_argument(<span class=\&quot;hljs-string\&quot;>&amp;#x27;--no-template&amp;#x27;</span>, action=<span class=\&quot;hljs-string\&quot;>&amp;#x27;store_true&amp;#x27;</span>,&quot;,&quot;                       <span class=\&quot;hljs-built_in\&quot;>help</span>=<span class=\&quot;hljs-string\&quot;>&amp;#x27;Prefill the question itself and start inference directly without chat template&amp;#x27;</span>)&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Add eval mode flag</span>&quot;,&quot;    parser.add_argument(<span class=\&quot;hljs-string\&quot;>&amp;#x27;--eval&amp;#x27;</span>, action=<span class=\&quot;hljs-string\&quot;>&amp;#x27;store_true&amp;#x27;</span>,&quot;,&quot;                       <span class=\&quot;hljs-built_in\&quot;>help</span>=<span class=\&quot;hljs-string\&quot;>&amp;#x27;Evaluation mode: suppress all output except model response&amp;#x27;</span>)&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Model configuration</span>&quot;,&quot;    parser.add_argument(<span class=\&quot;hljs-string\&quot;>&amp;#x27;--context-length&amp;#x27;</span>, <span class=\&quot;hljs-built_in\&quot;>type</span>=<span class=\&quot;hljs-built_in\&quot;>int</span>,&quot;,&quot;                       <span class=\&quot;hljs-built_in\&quot;>help</span>=<span class=\&quot;hljs-string\&quot;>&amp;#x27;Context length for the model (default: 512), if not provided, it will be detected from the model directory name ctxNUMBER&amp;#x27;</span>)&quot;,&quot;    parser.add_argument(<span class=\&quot;hljs-string\&quot;>&amp;#x27;--batch-size&amp;#x27;</span>, <span class=\&quot;hljs-built_in\&quot;>type</span>=<span class=\&quot;hljs-built_in\&quot;>int</span>,&quot;,&quot;                       <span class=\&quot;hljs-built_in\&quot;>help</span>=<span class=\&quot;hljs-string\&quot;>&amp;#x27;Batch size for prefill (default: 64)&amp;#x27;</span>)&quot;,&quot;    parser.add_argument(<span class=\&quot;hljs-string\&quot;>&amp;#x27;--num-logits&amp;#x27;</span>, <span class=\&quot;hljs-built_in\&quot;>type</span>=<span class=\&quot;hljs-built_in\&quot;>int</span>, default=<span class=\&quot;hljs-number\&quot;>8</span>,&quot;,&quot;                       <span class=\&quot;hljs-built_in\&quot;>help</span>=<span class=\&quot;hljs-string\&quot;>&amp;#x27;Number of logits outputs from LM head (default: 8, legacy)&amp;#x27;</span>)&quot;,&quot;    parser.add_argument(<span class=\&quot;hljs-string\&quot;>&amp;#x27;--split-lm-head&amp;#x27;</span>, <span class=\&quot;hljs-built_in\&quot;>type</span>=<span class=\&quot;hljs-built_in\&quot;>int</span>, &quot;,&quot;                       <span class=\&quot;hljs-built_in\&quot;>help</span>=<span class=\&quot;hljs-string\&quot;>&amp;#x27;Number of logits splits from LM head (default: 8 for llama, 16 for qwen)&amp;#x27;</span>)&quot;,&quot;    &quot;,&quot;    args = parser.parse_args()&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># If meta.yaml is provided, load parameters from it</span>&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>if</span> args.meta:&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>try</span>:&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>with</span> <span class=\&quot;hljs-built_in\&quot;>open</span>(args.meta, <span class=\&quot;hljs-string\&quot;>&amp;#x27;r&amp;#x27;</span>) <span class=\&quot;hljs-keyword\&quot;>as</span> f:&quot;,&quot;                meta = yaml.safe_load(f)&quot;,&quot;            params = meta[<span class=\&quot;hljs-string\&quot;>&amp;#x27;model_info&amp;#x27;</span>][<span class=\&quot;hljs-string\&quot;>&amp;#x27;parameters&amp;#x27;</span>]&quot;,&quot;            &quot;,&quot;            <span class=\&quot;hljs-comment\&quot;># Set model directory to meta.yaml directory if not specified</span>&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.d <span class=\&quot;hljs-keyword\&quot;>or</span> args.d == <span class=\&quot;hljs-string\&quot;>&amp;#x27;.&amp;#x27;</span>:&quot;,&quot;                args.d = <span class=\&quot;hljs-built_in\&quot;>str</span>(Path(args.meta).parent)&quot;,&quot;            &quot;,&quot;            <span class=\&quot;hljs-comment\&quot;># Build model paths based on parameters</span>&quot;,&quot;            prefix = params.get(<span class=\&quot;hljs-string\&quot;>&amp;#x27;model_prefix&amp;#x27;</span>, <span class=\&quot;hljs-string\&quot;>&amp;#x27;llama&amp;#x27;</span>)  <span class=\&quot;hljs-comment\&quot;># Default to &amp;#x27;llama&amp;#x27; if not specified</span>&quot;,&quot;            lut_ffn = <span class=\&quot;hljs-string\&quot;>f&amp;quot;_lut<span class=\&quot;hljs-subst\&quot;>{params[<span class=\&quot;hljs-string\&quot;>&amp;#x27;lut_ffn&amp;#x27;</span>]}</span>&amp;quot;</span> <span class=\&quot;hljs-keyword\&quot;>if</span> params[<span class=\&quot;hljs-string\&quot;>&amp;#x27;lut_ffn&amp;#x27;</span>] != <span class=\&quot;hljs-string\&quot;>&amp;#x27;none&amp;#x27;</span> <span class=\&quot;hljs-keyword\&quot;>else</span> <span class=\&quot;hljs-string\&quot;>&amp;#x27;&amp;#x27;</span>&quot;,&quot;            lut_lmhead = <span class=\&quot;hljs-string\&quot;>f&amp;quot;_lut<span class=\&quot;hljs-subst\&quot;>{params[<span class=\&quot;hljs-string\&quot;>&amp;#x27;lut_lmhead&amp;#x27;</span>]}</span>&amp;quot;</span> <span class=\&quot;hljs-keyword\&quot;>if</span> params[<span class=\&quot;hljs-string\&quot;>&amp;#x27;lut_lmhead&amp;#x27;</span>] != <span class=\&quot;hljs-string\&quot;>&amp;#x27;none&amp;#x27;</span> <span class=\&quot;hljs-keyword\&quot;>else</span> <span class=\&quot;hljs-string\&quot;>&amp;#x27;&amp;#x27;</span>&quot;,&quot;            lut_embeddings = <span class=\&quot;hljs-string\&quot;>f&amp;quot;_lut<span class=\&quot;hljs-subst\&quot;>{params[<span class=\&quot;hljs-string\&quot;>&amp;#x27;lut_embeddings&amp;#x27;</span>]}</span>&amp;quot;</span> <span class=\&quot;hljs-keyword\&quot;>if</span> params[<span class=\&quot;hljs-string\&quot;>&amp;#x27;lut_embeddings&amp;#x27;</span>] != <span class=\&quot;hljs-string\&quot;>&amp;#x27;none&amp;#x27;</span> <span class=\&quot;hljs-keyword\&quot;>else</span> <span class=\&quot;hljs-string\&quot;>&amp;#x27;&amp;#x27;</span>&quot;,&quot;            num_chunks = <span class=\&quot;hljs-built_in\&quot;>int</span>(params[<span class=\&quot;hljs-string\&quot;>&amp;#x27;num_chunks&amp;#x27;</span>])&quot;,&quot;            &quot;,&quot;            <span class=\&quot;hljs-comment\&quot;># Set model paths if not specified</span>&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.lmhead:&quot;,&quot;                args.lmhead = <span class=\&quot;hljs-string\&quot;>f&amp;#x27;<span class=\&quot;hljs-subst\&quot;>{prefix}</span>_lm_head<span class=\&quot;hljs-subst\&quot;>{lut_lmhead}</span>&amp;#x27;</span>&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.embed:&quot;,&quot;                args.embed = <span class=\&quot;hljs-string\&quot;>f&amp;#x27;<span class=\&quot;hljs-subst\&quot;>{prefix}</span>_embeddings<span class=\&quot;hljs-subst\&quot;>{lut_embeddings}</span>&amp;#x27;</span>  <span class=\&quot;hljs-comment\&quot;># Changed from lm_head to embeddings</span>&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.ffn:&quot;,&quot;                args.ffn = <span class=\&quot;hljs-string\&quot;>f&amp;#x27;<span class=\&quot;hljs-subst\&quot;>{prefix}</span>_FFN_PF<span class=\&quot;hljs-subst\&quot;>{lut_ffn}</span>_chunk_01of<span class=\&quot;hljs-subst\&quot;>{num_chunks:02d}</span>&amp;#x27;</span>&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.tokenizer:&quot;,&quot;                <span class=\&quot;hljs-comment\&quot;># Check if there&amp;#x27;s a tokenizer_path parameter in meta.yaml</span>&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-string\&quot;>&amp;#x27;tokenizer_path&amp;#x27;</span> <span class=\&quot;hljs-keyword\&quot;>in</span> params:&quot;,&quot;                    args.tokenizer = params[<span class=\&quot;hljs-string\&quot;>&amp;#x27;tokenizer_path&amp;#x27;</span>]&quot;,&quot;                <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;                    <span class=\&quot;hljs-comment\&quot;># Default to the model directory, but this might need manual override</span>&quot;,&quot;                    args.tokenizer = args.d&quot;,&quot;            &quot;,&quot;            <span class=\&quot;hljs-comment\&quot;># Set other parameters if not overridden by command line</span>&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> args.context_length <span class=\&quot;hljs-keyword\&quot;>is</span> <span class=\&quot;hljs-literal\&quot;>None</span>:&quot;,&quot;                args.context_length = <span class=\&quot;hljs-built_in\&quot;>int</span>(params[<span class=\&quot;hljs-string\&quot;>&amp;#x27;context_length&amp;#x27;</span>])&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> args.batch_size <span class=\&quot;hljs-keyword\&quot;>is</span> <span class=\&quot;hljs-literal\&quot;>None</span>:&quot;,&quot;                args.batch_size = <span class=\&quot;hljs-built_in\&quot;>int</span>(params[<span class=\&quot;hljs-string\&quot;>&amp;#x27;batch_size&amp;#x27;</span>])&quot;,&quot;            args.num_chunks = num_chunks&quot;,&quot;            <span class=\&quot;hljs-comment\&quot;># Add num_logits parameter with default of 8, override command line if present in meta</span>&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-string\&quot;>&amp;#x27;num_logits&amp;#x27;</span> <span class=\&quot;hljs-keyword\&quot;>in</span> params:&quot;,&quot;                args.num_logits = <span class=\&quot;hljs-built_in\&quot;>int</span>(params[<span class=\&quot;hljs-string\&quot;>&amp;#x27;num_logits&amp;#x27;</span>])&quot;,&quot;            &quot;,&quot;            <span class=\&quot;hljs-comment\&quot;># Add split_lm_head parameter with default of 8</span>&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-string\&quot;>&amp;#x27;split_lm_head&amp;#x27;</span> <span class=\&quot;hljs-keyword\&quot;>in</span> params:&quot;,&quot;                args.split_lm_head = <span class=\&quot;hljs-built_in\&quot;>int</span>(params[<span class=\&quot;hljs-string\&quot;>&amp;#x27;split_lm_head&amp;#x27;</span>])&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;                args.split_lm_head = <span class=\&quot;hljs-number\&quot;>8</span>  <span class=\&quot;hljs-comment\&quot;># Default value for backward compatibility</span>&quot;,&quot;            &quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nLoaded parameters from <span class=\&quot;hljs-subst\&quot;>{args.meta}</span>:&amp;quot;</span>)&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  Context Length: <span class=\&quot;hljs-subst\&quot;>{args.context_length}</span>&amp;quot;</span>)&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  Batch Size: <span class=\&quot;hljs-subst\&quot;>{args.batch_size}</span>&amp;quot;</span>)&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  Num Chunks: <span class=\&quot;hljs-subst\&quot;>{args.num_chunks}</span>&amp;quot;</span>)&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  Num Logits: <span class=\&quot;hljs-subst\&quot;>{args.num_logits}</span>&amp;quot;</span>)&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  Split LM Head: <span class=\&quot;hljs-subst\&quot;>{args.split_lm_head}</span>&amp;quot;</span>)&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  Models Directory: <span class=\&quot;hljs-subst\&quot;>{args.d}</span>&amp;quot;</span>)&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  Embeddings: <span class=\&quot;hljs-subst\&quot;>{args.embed}</span>&amp;quot;</span>)&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  LM Head: <span class=\&quot;hljs-subst\&quot;>{args.lmhead}</span>&amp;quot;</span>)&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;  FFN: <span class=\&quot;hljs-subst\&quot;>{args.ffn}</span>&amp;quot;</span>)&quot;,&quot;            &quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>except</span> Exception <span class=\&quot;hljs-keyword\&quot;>as</span> e:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nError loading meta.yaml: <span class=\&quot;hljs-subst\&quot;>{<span class=\&quot;hljs-built_in\&quot;>str</span>(e)}</span>&amp;quot;</span>)&quot;,&quot;            sys.exit(<span class=\&quot;hljs-number\&quot;>1</span>)&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>else</span>:&quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># If no meta.yaml, set default split_lm_head if not provided</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> <span class=\&quot;hljs-built_in\&quot;>hasattr</span>(args, <span class=\&quot;hljs-string\&quot;>&amp;#x27;split_lm_head&amp;#x27;</span>) <span class=\&quot;hljs-keyword\&quot;>or</span> args.split_lm_head <span class=\&quot;hljs-keyword\&quot;>is</span> <span class=\&quot;hljs-literal\&quot;>None</span>:&quot;,&quot;            args.split_lm_head = args.num_logits  <span class=\&quot;hljs-comment\&quot;># Use num_logits as fallback</span>&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>return</span> args&quot;,&quot;&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>def</span> <span class=\&quot;hljs-title function_\&quot;>main</span>():&quot;,&quot;    args = parse_args()&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-comment\&quot;># Convert directory to absolute path</span>&quot;,&quot;    model_dir = Path(args.d).resolve()&quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> model_dir.exists():&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nError: Model directory not found: <span class=\&quot;hljs-subst\&quot;>{model_dir}</span>&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>return</span> <span class=\&quot;hljs-number\&quot;>1</span>&quot;,&quot;        &quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nUsing model directory: <span class=\&quot;hljs-subst\&quot;>{model_dir}</span>&amp;quot;</span>)&quot;,&quot;        <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;Context length: <span class=\&quot;hljs-subst\&quot;>{args.context_length}</span>&amp;quot;</span>)&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>try</span>:&quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Update paths to be relative to model directory</span>&quot;,&quot;        args.embed = <span class=\&quot;hljs-built_in\&quot;>str</span>(model_dir / args.embed)&quot;,&quot;        args.ffn = <span class=\&quot;hljs-built_in\&quot;>str</span>(model_dir / args.ffn)&quot;,&quot;        args.lmhead = <span class=\&quot;hljs-built_in\&quot;>str</span>(model_dir / args.lmhead)&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Handle tokenizer path separately since it&amp;#x27;s not relative to model_dir</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> args.tokenizer <span class=\&quot;hljs-keyword\&quot;>is</span> <span class=\&quot;hljs-literal\&quot;>None</span>:&quot;,&quot;            args.tokenizer = <span class=\&quot;hljs-built_in\&quot;>str</span>(model_dir)&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Check if tokenizer directory exists and has required files</span>&quot;,&quot;        tokenizer_path = Path(args.tokenizer)&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> tokenizer_path.exists():&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nError: Tokenizer directory not found: <span class=\&quot;hljs-subst\&quot;>{args.tokenizer}</span>&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>return</span> <span class=\&quot;hljs-number\&quot;>1</span>&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Check if tokenizer has the required files</span>&quot;,&quot;        required_files = [<span class=\&quot;hljs-string\&quot;>&amp;#x27;tokenizer.json&amp;#x27;</span>, <span class=\&quot;hljs-string\&quot;>&amp;#x27;tokenizer_config.json&amp;#x27;</span>]&quot;,&quot;        missing_files = [f <span class=\&quot;hljs-keyword\&quot;>for</span> f <span class=\&quot;hljs-keyword\&quot;>in</span> required_files <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> (tokenizer_path / f).exists()]&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> missing_files <span class=\&quot;hljs-keyword\&quot;>and</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nWarning: Tokenizer directory missing required files: <span class=\&quot;hljs-subst\&quot;>{missing_files}</span>&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;Current tokenizer path: <span class=\&quot;hljs-subst\&quot;>{args.tokenizer}</span>&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nFor Qwen models, you may need to specify the original model directory:&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;  python chat.py --meta /tmp/qwen/meta.yaml --tokenizer ~/.cache/huggingface/hub/models--Qwen--Qwen3-0.6B/snapshots/YOUR_SNAPSHOT_ID&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>&amp;quot;\\nOr add &amp;#x27;tokenizer_path&amp;#x27; to your meta.yaml file.&amp;quot;</span>)&quot;,&quot;    &quot;,&quot;        args.tokenizer = <span class=\&quot;hljs-built_in\&quot;>str</span>(Path(args.tokenizer).resolve())  <span class=\&quot;hljs-comment\&quot;># Convert to absolute path</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;Using tokenizer path: <span class=\&quot;hljs-subst\&quot;>{args.tokenizer}</span>&amp;quot;</span>)&quot;,&quot;        &quot;,&quot;        metadata = {}&quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Load models and extract metadata</span>&quot;,&quot;        embed_model, ffn_models, lmhead_model, metadata = load_models(args,metadata)&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nMetadata befor args.context_length: <span class=\&quot;hljs-subst\&quot;>{metadata}</span>&amp;quot;</span>)&quot;,&quot;&quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Override context length from command line if provided</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> args.context_length <span class=\&quot;hljs-keyword\&quot;>is</span> <span class=\&quot;hljs-keyword\&quot;>not</span> <span class=\&quot;hljs-literal\&quot;>None</span>:&quot;,&quot;            metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;context_length&amp;#x27;</span>] = args.context_length&quot;,&quot;            metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;state_length&amp;#x27;</span>] = args.context_length  <span class=\&quot;hljs-comment\&quot;># Also update state_length</span>&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;                <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nOverriding context length from command line: <span class=\&quot;hljs-subst\&quot;>{args.context_length}</span>&amp;quot;</span>)&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Add num_logits to metadata (legacy support)</span>&quot;,&quot;        metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;num_logits&amp;#x27;</span>] = <span class=\&quot;hljs-built_in\&quot;>getattr</span>(args, <span class=\&quot;hljs-string\&quot;>&amp;#x27;num_logits&amp;#x27;</span>, <span class=\&quot;hljs-number\&quot;>8</span>)&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Add split_lm_head to metadata (preferred)</span>&quot;,&quot;        metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;split_lm_head&amp;#x27;</span>] = <span class=\&quot;hljs-built_in\&quot;>getattr</span>(args, <span class=\&quot;hljs-string\&quot;>&amp;#x27;split_lm_head&amp;#x27;</span>, <span class=\&quot;hljs-built_in\&quot;>getattr</span>(args, <span class=\&quot;hljs-string\&quot;>&amp;#x27;num_logits&amp;#x27;</span>, <span class=\&quot;hljs-number\&quot;>8</span>))&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nMetadata after load_models: <span class=\&quot;hljs-subst\&quot;>{metadata}</span>&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;Using split_lm_head value: <span class=\&quot;hljs-subst\&quot;>{metadata.get(<span class=\&quot;hljs-string\&quot;>&amp;#x27;split_lm_head&amp;#x27;</span>, <span class=\&quot;hljs-number\&quot;>8</span>)}</span>&amp;quot;</span>)&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Load tokenizer with resolved path</span>&quot;,&quot;        tokenizer = initialize_tokenizer(args.tokenizer, args.<span class=\&quot;hljs-built_in\&quot;>eval</span>)&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> tokenizer <span class=\&quot;hljs-keyword\&quot;>is</span> <span class=\&quot;hljs-literal\&quot;>None</span>:&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>raise</span> RuntimeError(<span class=\&quot;hljs-string\&quot;>&amp;quot;Failed to initialize tokenizer&amp;quot;</span>)&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Create unified state once</span>&quot;,&quot;        state = create_unified_state(ffn_models, metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;context_length&amp;#x27;</span>], args.<span class=\&quot;hljs-built_in\&quot;>eval</span>)&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Initialize causal mask once</span>&quot;,&quot;        causal_mask = initialize_causal_mask(metadata[<span class=\&quot;hljs-string\&quot;>&amp;#x27;context_length&amp;#x27;</span>], args.<span class=\&quot;hljs-built_in\&quot;>eval</span>)&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Warmup runs to prevent Python GIL issues with CoreML !</span>&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.nw <span class=\&quot;hljs-keyword\&quot;>and</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>for</span> _ <span class=\&quot;hljs-keyword\&quot;>in</span> <span class=\&quot;hljs-built_in\&quot;>range</span>(<span class=\&quot;hljs-number\&quot;>2</span>):&quot;,&quot;                chat_loop(&quot;,&quot;                    embed_model=embed_model,&quot;,&quot;                    ffn_models=ffn_models,&quot;,&quot;                    lmhead_model=lmhead_model,&quot;,&quot;                    tokenizer=tokenizer,&quot;,&quot;                    metadata=metadata,&quot;,&quot;                    state=state,&quot;,&quot;                    causal_mask=causal_mask,  <span class=\&quot;hljs-comment\&quot;># Pass the causal mask</span>&quot;,&quot;                    warmup=<span class=\&quot;hljs-literal\&quot;>True</span>,&quot;,&quot;                    auto_prompt=<span class=\&quot;hljs-string\&quot;>&amp;quot;who are you?&amp;quot;</span>,&quot;,&quot;                    no_template=args.no_template,&quot;,&quot;                    eval_mode=args.<span class=\&quot;hljs-built_in\&quot;>eval</span>&quot;,&quot;                )&quot;,&quot;        &quot;,&quot;        <span class=\&quot;hljs-comment\&quot;># Main run</span>&quot;,&quot;        chat_loop(&quot;,&quot;            embed_model=embed_model,&quot;,&quot;            ffn_models=ffn_models,&quot;,&quot;            lmhead_model=lmhead_model,&quot;,&quot;            tokenizer=tokenizer,&quot;,&quot;            metadata=metadata,&quot;,&quot;            state=state,&quot;,&quot;            causal_mask=causal_mask,  <span class=\&quot;hljs-comment\&quot;># Pass the causal mask</span>&quot;,&quot;            warmup=<span class=\&quot;hljs-literal\&quot;>False</span>,&quot;,&quot;            auto_prompt=args.prompt,&quot;,&quot;            save_file=args.save,&quot;,&quot;            max_tokens=args.max_tokens,&quot;,&quot;            no_template=args.no_template,&quot;,&quot;            eval_mode=args.<span class=\&quot;hljs-built_in\&quot;>eval</span>&quot;,&quot;        )&quot;,&quot;        &quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>except</span> Exception <span class=\&quot;hljs-keyword\&quot;>as</span> e:&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>if</span> <span class=\&quot;hljs-keyword\&quot;>not</span> args.<span class=\&quot;hljs-built_in\&quot;>eval</span>:&quot;,&quot;            <span class=\&quot;hljs-built_in\&quot;>print</span>(<span class=\&quot;hljs-string\&quot;>f&amp;quot;\\nError: <span class=\&quot;hljs-subst\&quot;>{<span class=\&quot;hljs-built_in\&quot;>str</span>(e)}</span>&amp;quot;</span>)&quot;,&quot;            <span class=\&quot;hljs-keyword\&quot;>import</span> traceback&quot;,&quot;            traceback.print_exc()&quot;,&quot;        <span class=\&quot;hljs-keyword\&quot;>return</span> <span class=\&quot;hljs-number\&quot;>1</span>&quot;,&quot;    &quot;,&quot;    <span class=\&quot;hljs-keyword\&quot;>return</span> <span class=\&quot;hljs-number\&quot;>0</span>&quot;,&quot;&quot;,&quot;<span class=\&quot;hljs-keyword\&quot;>if</span> __name__ == <span class=\&quot;hljs-string\&quot;>&amp;quot;__main__&amp;quot;</span>:&quot;,&quot;    exit(main()) &quot;,&quot;&quot;],&quot;lineSelectorClass&quot;:&quot;blob-line&quot;,&quot;context&quot;:{&quot;repo&quot;:{&quot;name&quot;:&quot;anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4&quot;,&quot;type&quot;:&quot;model&quot;},&quot;rev&quot;:&quot;main&quot;,&quot;path&quot;:&quot;chat.py&quot;,&quot;subpaths&quot;:[{&quot;dir&quot;:&quot;chat.py&quot;}]}}">

<div class="@container relative"><div class="@max-3xl:text-xs overflow-x-auto text-sm"><table class="min-w-full border-collapse font-mono"><tbody>
					<tr class="" id="L1">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-comment"># chat.py</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L2">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="2">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-comment">#!/usr/bin/env python3</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L3">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="3">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-comment"># chat.py</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L4">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="4">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-comment"># Copyright (c) 2025 Anemll</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L5">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="5">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-comment"># Licensed under the MIT License</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L6">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="6">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L7">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="7">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">import</span> argparse<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L8">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="8">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">import</span> os<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L9">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="9">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">import</span> re<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L10">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="10">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">import</span> glob<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L11">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="11">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">from</span> pathlib <span class="hljs-keyword">import</span> Path<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L12">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="12">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">import</span> coremltools <span class="hljs-keyword">as</span> ct<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L13">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="13">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">from</span> transformers <span class="hljs-keyword">import</span> LlamaTokenizer, AutoTokenizer<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L14">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="14">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">import</span> torch<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L15">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="15">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">import</span> torch.nn.functional <span class="hljs-keyword">as</span> F<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L16">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="16">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">import</span> numpy <span class="hljs-keyword">as</span> np<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L17">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="17">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">import</span> queue<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L18">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="18">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">import</span> threading<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L19">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="19">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">import</span> time<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L20">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="20">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">import</span> yaml<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L21">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="21">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">import</span> sys<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L22">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="22">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L23">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="23">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-comment"># ANSI color codes</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L24">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="24">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->LIGHT_BLUE = <span class="hljs-string">&quot;\033[94m&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L25">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="25">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->DARK_BLUE = <span class="hljs-string">&quot;\033[34m&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L26">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="26">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->LIGHT_GREEN = <span class="hljs-string">&quot;\033[92m&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L27">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="27">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->RESET_COLOR = <span class="hljs-string">&quot;\033[0m&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L28">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="28">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L29">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="29">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-comment"># Add at top with other constants</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L30">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="30">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->WARMUP_TOKEN_LIMIT = <span class="hljs-number">10</span>  <span class="hljs-comment"># Maximum tokens to generate during warmup</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L31">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="31">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L32">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="32">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">class</span> <span class="hljs-title class_">TokenPrinter</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L33">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="33">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-string">&quot;&quot;&quot;Handles background printing of generated tokens.&quot;&quot;&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L34">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="34">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">def</span> <span class="hljs-title function_">__init__</span>(<span class="hljs-params">self, tokenizer</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L35">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="35">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        self.tokenizer = tokenizer<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L36">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="36">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        self.token_queue = queue.Queue()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L37">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="37">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        self.stop_event = threading.Event()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L38">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="38">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        self.thread = <span class="hljs-literal">None</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L39">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="39">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        self.buffer = <span class="hljs-string">&quot;&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L40">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="40">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        self.lock = threading.Lock()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L41">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="41">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        self.thinking = <span class="hljs-literal">True</span>  <span class="hljs-comment"># Track if we&#x27;re still in thinking mode</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L42">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="42">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        self.decoding_buffer = []  <span class="hljs-comment"># Buffer for token IDs</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L43">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="43">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Add token counting and timing</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L44">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="44">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        self.start_time = time.time()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L45">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="45">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        self.token_count = <span class="hljs-number">0</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L46">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="46">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        self.start()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L47">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="47">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L48">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="48">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">def</span> <span class="hljs-title function_">start</span>(<span class="hljs-params">self</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L49">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="49">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-string">&quot;&quot;&quot;Start the printer thread.&quot;&quot;&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L50">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="50">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> self.thread <span class="hljs-keyword">is</span> <span class="hljs-literal">None</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L51">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="51">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            self.thread = threading.Thread(target=self._print_worker)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L52">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="52">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            self.thread.daemon = <span class="hljs-literal">True</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L53">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="53">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            self.thread.start()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L54">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="54">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L55">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="55">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">def</span> <span class="hljs-title function_">add_token</span>(<span class="hljs-params">self, token_id</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L56">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="56">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-string">&quot;&quot;&quot;Add a token to the print queue.&quot;&quot;&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L57">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="57">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> self.stop_event.is_set():<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L58">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="58">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            self.token_queue.put(token_id)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L59">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="59">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            self.token_count += <span class="hljs-number">1</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L60">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="60">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L61">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="61">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">def</span> <span class="hljs-title function_">drain_buffer</span>(<span class="hljs-params">self, eval_mode=<span class="hljs-literal">False</span></span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L62">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="62">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-string">&quot;&quot;&quot;Decode token IDs from decoding_buffer in the main thread.&quot;&quot;&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L63">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="63">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> self.decoding_buffer:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L64">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="64">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">return</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L65">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="65">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L66">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="66">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Decode all tokens at once in the main thread</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L67">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="67">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        token_str = self.tokenizer.decode(self.decoding_buffer)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L68">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="68">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        self.decoding_buffer.clear()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L69">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="69">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L70">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="70">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Store the text in buffer for later saving to file</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L71">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="71">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">with</span> self.lock:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L72">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="72">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            self.buffer += token_str<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L73">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="73">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L74">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="74">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Skip printing in eval mode</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L75">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="75">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> eval_mode:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L76">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="76">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">return</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L77">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="77">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L78">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="78">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Color-handling logic</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L79">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="79">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> self.thinking <span class="hljs-keyword">and</span> <span class="hljs-string">&quot;&lt;/think&gt;&quot;</span> <span class="hljs-keyword">in</span> token_str:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L80">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="80">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            self.thinking = <span class="hljs-literal">False</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L81">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="81">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            parts = token_str.split(<span class="hljs-string">&quot;&lt;/think&gt;&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L82">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="82">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-built_in">len</span>(parts) &gt; <span class="hljs-number">0</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L83">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="83">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(parts[<span class="hljs-number">0</span>] + <span class="hljs-string">&quot;&lt;/think&gt;&quot;</span>, end=<span class="hljs-string">&#x27;&#x27;</span>, flush=<span class="hljs-literal">True</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L84">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="84">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">if</span> <span class="hljs-built_in">len</span>(parts) &gt; <span class="hljs-number">1</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L85">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="85">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-built_in">print</span>(LIGHT_BLUE + parts[<span class="hljs-number">1</span>], end=<span class="hljs-string">&#x27;&#x27;</span>, flush=<span class="hljs-literal">True</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L86">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="86">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L87">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="87">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> self.thinking:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L88">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="88">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(LIGHT_BLUE + token_str, end=<span class="hljs-string">&#x27;&#x27;</span>, flush=<span class="hljs-literal">True</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L89">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="89">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L90">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="90">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(token_str, end=<span class="hljs-string">&#x27;&#x27;</span>, flush=<span class="hljs-literal">True</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L91">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="91">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L92">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="92">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">def</span> <span class="hljs-title function_">_print_worker</span>(<span class="hljs-params">self</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L93">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="93">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-string">&quot;&quot;&quot;Worker thread that takes token_ids from the queue.&quot;&quot;&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L94">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="94">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">while</span> <span class="hljs-keyword">not</span> self.stop_event.is_set():<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L95">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="95">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">try</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L96">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="96">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                token_id = self.token_queue.get(timeout=<span class="hljs-number">0.01</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L97">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="97">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">with</span> self.lock:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L98">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="98">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    self.decoding_buffer.append(token_id)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L99">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="99">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                self.token_queue.task_done()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L100">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="100">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">except</span> queue.Empty:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L101">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="101">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">continue</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L102">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="102">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">except</span> Exception <span class="hljs-keyword">as</span> e:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L103">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="103">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nError: Token printer error: <span class="hljs-subst">{<span class="hljs-built_in">str</span>(e)}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L104">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="104">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">break</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L105">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="105">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L106">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="106">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">def</span> <span class="hljs-title function_">stop</span>(<span class="hljs-params">self, eval_mode=<span class="hljs-literal">False</span></span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L107">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="107">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-string">&quot;&quot;&quot;Stop the printer thread.&quot;&quot;&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L108">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="108">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> self.thread <span class="hljs-keyword">and</span> self.thread.is_alive():<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L109">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="109">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-comment"># Ensure any remaining tokens are processed</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L110">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="110">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            self.drain_buffer()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L111">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="111">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            self.stop_event.<span class="hljs-built_in">set</span>()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L112">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="112">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">try</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L113">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="113">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                self.thread.join(timeout=<span class="hljs-number">1.0</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L114">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="114">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">except</span> Exception:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L115">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="115">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">pass</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L116">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="116">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-comment"># Calculate and print tokens/s with shorter format in blue (unless in eval mode)</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L117">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="117">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> eval_mode:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L118">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="118">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                elapsed = time.time() - self.start_time<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L119">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="119">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">if</span> elapsed &gt; <span class="hljs-number">0</span> <span class="hljs-keyword">and</span> self.token_count &gt; <span class="hljs-number">0</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L120">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="120">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    tokens_per_sec = self.token_count / elapsed<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L121">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="121">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\n<span class="hljs-subst">{DARK_BLUE}</span><span class="hljs-subst">{tokens_per_sec:<span class="hljs-number">.1</span>f}</span> t/s<span class="hljs-subst">{RESET_COLOR}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L122">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="122">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L123">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="123">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-built_in">print</span>(RESET_COLOR)  <span class="hljs-comment"># Reset color at the end</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L124">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="124">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">return</span> self.buffer<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L125">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="125">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L126">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="126">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">def</span> <span class="hljs-title function_">parse_model_path</span>(<span class="hljs-params">path</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L127">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="127">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-string">&quot;&quot;&quot;Parse model path and return full path with .mlmodelc or .mlpackage extension.&quot;&quot;&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L128">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="128">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    path = Path(path)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L129">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="129">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L130">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="130">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># If path exists exactly as specified, return it</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L131">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="131">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">if</span> path.exists():<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L132">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="132">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">return</span> <span class="hljs-built_in">str</span>(path)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L133">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="133">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L134">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="134">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Try with both extensions</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L135">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="135">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    candidates = [<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L136">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="136">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        path,  <span class="hljs-comment"># Original path</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L137">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="137">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        path.with_suffix(<span class="hljs-string">&#x27;.mlmodelc&#x27;</span>),  <span class="hljs-comment"># With .mlmodelc</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L138">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="138">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        path.with_suffix(<span class="hljs-string">&#x27;.mlpackage&#x27;</span>),  <span class="hljs-comment"># With .mlpackage</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L139">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="139">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        Path(<span class="hljs-built_in">str</span>(path) + <span class="hljs-string">&#x27;.mlmodelc&#x27;</span>),  <span class="hljs-comment"># Handle case where extension is included</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L140">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="140">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        Path(<span class="hljs-built_in">str</span>(path) + <span class="hljs-string">&#x27;.mlpackage&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L141">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="141">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    ]<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L142">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="142">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L143">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="143">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Try all possible paths</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L144">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="144">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">for</span> candidate <span class="hljs-keyword">in</span> candidates:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L145">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="145">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> candidate.exists():<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L146">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="146">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">return</span> <span class="hljs-built_in">str</span>(candidate)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L147">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="147">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L148">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="148">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># If embeddings with LUT suffix not found, try without LUT suffix</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L149">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="149">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">if</span> <span class="hljs-string">&quot;_lut&quot;</span> <span class="hljs-keyword">in</span> <span class="hljs-built_in">str</span>(path) <span class="hljs-keyword">and</span> <span class="hljs-string">&quot;embeddings&quot;</span> <span class="hljs-keyword">in</span> <span class="hljs-built_in">str</span>(path):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L150">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="150">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;Failed to find <span class="hljs-subst">{path}</span>, trying without LUT suffix...&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L151">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="151">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Remove LUT suffix</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L152">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="152">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        path_no_lut = <span class="hljs-built_in">str</span>(path).split(<span class="hljs-string">&quot;_lut&quot;</span>)[<span class="hljs-number">0</span>]<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L153">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="153">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        path_no_lut = Path(path_no_lut)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L154">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="154">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L155">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="155">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Try candidates without LUT suffix</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L156">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="156">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        candidates_no_lut = [<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L157">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="157">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            path_no_lut,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L158">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="158">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            path_no_lut.with_suffix(<span class="hljs-string">&#x27;.mlmodelc&#x27;</span>),<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L159">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="159">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            path_no_lut.with_suffix(<span class="hljs-string">&#x27;.mlpackage&#x27;</span>),<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L160">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="160">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            Path(<span class="hljs-built_in">str</span>(path_no_lut) + <span class="hljs-string">&#x27;.mlmodelc&#x27;</span>),<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L161">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="161">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            Path(<span class="hljs-built_in">str</span>(path_no_lut) + <span class="hljs-string">&#x27;.mlpackage&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L162">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="162">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        ]<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L163">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="163">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L164">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="164">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">for</span> candidate <span class="hljs-keyword">in</span> candidates_no_lut:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L165">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="165">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> candidate.exists():<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L166">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="166">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">return</span> <span class="hljs-built_in">str</span>(candidate)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L167">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="167">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L168">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="168">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Add no-LUT candidates to the list for error reporting</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L169">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="169">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        candidates.extend(candidates_no_lut)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L170">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="170">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L171">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="171">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># If we get here, no valid path was found</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L172">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="172">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nError: Model not found. Tried following paths:&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L173">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="173">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">for</span> candidate <span class="hljs-keyword">in</span> candidates:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L174">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="174">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  <span class="hljs-subst">{candidate}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L175">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="175">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">raise</span> FileNotFoundError(<span class="hljs-string">f&quot;Model not found: <span class="hljs-subst">{path}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L176">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="176">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L177">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="177">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">def</span> <span class="hljs-title function_">parse_ffn_filename</span>(<span class="hljs-params">path</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L178">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="178">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-string">&quot;&quot;&quot;Parse FFN model filename to extract chunk information.&quot;&quot;&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L179">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="179">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    path = Path(path)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L180">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="180">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    pattern = <span class="hljs-string">r&#x27;FFN_PF.*_chunk_(\d+)of(\d+)&#x27;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L181">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="181">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">match</span> = re.search(pattern, path.name)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L182">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="182">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L183">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="183">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">if</span> <span class="hljs-keyword">match</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L184">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="184">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        current_chunk = <span class="hljs-built_in">int</span>(<span class="hljs-keyword">match</span>.group(<span class="hljs-number">1</span>))<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L185">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="185">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        total_chunks = <span class="hljs-built_in">int</span>(<span class="hljs-keyword">match</span>.group(<span class="hljs-number">2</span>))<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L186">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="186">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">return</span> current_chunk, total_chunks<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L187">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="187">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">return</span> <span class="hljs-literal">None</span>, <span class="hljs-literal">None</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L188">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="188">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L189">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="189">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">def</span> <span class="hljs-title function_">find_all_chunks</span>(<span class="hljs-params">base_path</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L190">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="190">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-string">&quot;&quot;&quot;Find all chunk files matching the base FFN path pattern.&quot;&quot;&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L191">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="191">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    path = Path(base_path)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L192">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="192">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    pattern = re.sub(<span class="hljs-string">r&#x27;_chunk_\d+of\d+&#x27;</span>, <span class="hljs-string">&#x27;_chunk_*&#x27;</span>, <span class="hljs-built_in">str</span>(path))<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L193">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="193">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">return</span> <span class="hljs-built_in">sorted</span>(glob.glob(pattern))<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L194">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="194">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L195">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="195">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">def</span> <span class="hljs-title function_">load_model</span>(<span class="hljs-params">path, function_name=<span class="hljs-literal">None</span></span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L196">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="196">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-string">&quot;&quot;&quot;Load a CoreML model, handling both .mlmodelc and .mlpackage formats.&quot;&quot;&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L197">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="197">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    path = Path(path)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L198">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="198">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    compute_unit = ct.ComputeUnit.CPU_AND_NE<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L199">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="199">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L200">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="200">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">try</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L201">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="201">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> path.suffix == <span class="hljs-string">&#x27;.mlmodelc&#x27;</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L202">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="202">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-comment"># For compiled models (.mlmodelc), use CompiledMLModel</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L203">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="203">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> function_name:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L204">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="204">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">return</span> ct.models.CompiledMLModel(<span class="hljs-built_in">str</span>(path), compute_unit, function_name=function_name)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L205">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="205">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L206">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="206">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">return</span> ct.models.CompiledMLModel(<span class="hljs-built_in">str</span>(path), compute_unit)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L207">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="207">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L208">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="208">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-comment"># For packages (.mlpackage)</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L209">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="209">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> function_name:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L210">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="210">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">return</span> ct.models.MLModel(<span class="hljs-built_in">str</span>(path), function_name=function_name)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L211">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="211">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L212">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="212">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">return</span> ct.models.MLModel(<span class="hljs-built_in">str</span>(path))<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L213">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="213">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L214">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="214">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">except</span> RuntimeError <span class="hljs-keyword">as</span> e:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L215">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="215">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-string">&quot;valid manifest does not exist&quot;</span> <span class="hljs-keyword">in</span> <span class="hljs-built_in">str</span>(e):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L216">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="216">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nError: Could not load compiled model at <span class="hljs-subst">{path}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L217">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="217">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;This might be because:&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L218">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="218">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;1. The model is not properly compiled&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L219">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="219">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;2. The model was compiled for a different OS version&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L220">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="220">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;3. The model needs to be recompiled&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L221">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="221">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nTry using the .mlpackage version instead, or recompile the model.&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L222">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="222">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">raise</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L223">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="223">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L224">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="224">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">def</span> <span class="hljs-title function_">load_metadata</span>(<span class="hljs-params">model,args</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L225">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="225">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Extract metadata and config parameters</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L226">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="226">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    metadata = {}<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L227">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="227">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">if</span> <span class="hljs-built_in">hasattr</span>(model, <span class="hljs-string">&#x27;user_defined_metadata&#x27;</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L228">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="228">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        meta = model.user_defined_metadata<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L229">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="229">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L230">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="230">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Extract key parameters with defaults</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L231">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="231">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        metadata[<span class="hljs-string">&#x27;context_length&#x27;</span>] = <span class="hljs-built_in">int</span>(meta.get(<span class="hljs-string">&#x27;com.anemll.context_length&#x27;</span>, <span class="hljs-number">512</span>))<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L232">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="232">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        metadata[<span class="hljs-string">&#x27;state_length&#x27;</span>] = <span class="hljs-built_in">int</span>(meta.get(<span class="hljs-string">&#x27;com.anemll.state_length&#x27;</span>, metadata[<span class="hljs-string">&#x27;context_length&#x27;</span>]))  <span class="hljs-comment"># Added state_length</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L233">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="233">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        metadata[<span class="hljs-string">&#x27;batch_size&#x27;</span>] = <span class="hljs-built_in">int</span>(meta.get(<span class="hljs-string">&#x27;com.anemll.batch_size&#x27;</span>, <span class="hljs-number">64</span>))<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L234">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="234">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        metadata[<span class="hljs-string">&#x27;lut_bits&#x27;</span>] = <span class="hljs-built_in">int</span>(meta.get(<span class="hljs-string">&#x27;com.anemll.lut_bits&#x27;</span>, <span class="hljs-number">0</span>))<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L235">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="235">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        metadata[<span class="hljs-string">&#x27;num_chunks&#x27;</span>] = <span class="hljs-built_in">int</span>(meta.get(<span class="hljs-string">&#x27;com.anemll.num_chunks&#x27;</span>, <span class="hljs-number">1</span>))<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L236">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="236">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L237">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="237">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L238">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="238">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nExtracted Parameters:&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L239">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="239">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  Context Length: <span class="hljs-subst">{metadata[<span class="hljs-string">&#x27;context_length&#x27;</span>]}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L240">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="240">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  State Length: <span class="hljs-subst">{metadata[<span class="hljs-string">&#x27;state_length&#x27;</span>]}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L241">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="241">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  Prefill Batch Size: <span class="hljs-subst">{metadata[<span class="hljs-string">&#x27;batch_size&#x27;</span>]}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L242">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="242">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  LUT Bits: <span class="hljs-subst">{metadata[<span class="hljs-string">&#x27;lut_bits&#x27;</span>]}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L243">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="243">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  Number of Chunks: <span class="hljs-subst">{metadata[<span class="hljs-string">&#x27;num_chunks&#x27;</span>]}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L244">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="244">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L245">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="245">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-comment"># Print model info</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L246">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="246">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nModel Info:&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L247">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="247">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-string">&#x27;com.anemll.info&#x27;</span> <span class="hljs-keyword">in</span> meta:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L248">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="248">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  <span class="hljs-subst">{meta[<span class="hljs-string">&#x27;com.anemll.info&#x27;</span>]}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L249">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="249">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-string">&#x27;com.github.apple.coremltools.version&#x27;</span> <span class="hljs-keyword">in</span> meta:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L250">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="250">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  CoreML Tools: <span class="hljs-subst">{meta[<span class="hljs-string">&#x27;com.github.apple.coremltools.version&#x27;</span>]}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L251">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="251">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L252">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="252">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-comment"># Print model input/output shapes</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L253">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="253">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nModel Shapes:&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L254">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="254">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-built_in">hasattr</span>(model, <span class="hljs-string">&#x27;input_description&#x27;</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L255">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="255">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;  Inputs:&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L256">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="256">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">try</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L257">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="257">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-keyword">if</span> <span class="hljs-built_in">hasattr</span>(model.input_description, <span class="hljs-string">&#x27;items&#x27;</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L258">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="258">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-keyword">for</span> name, desc <span class="hljs-keyword">in</span> model.input_description.items():<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L259">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="259">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;    <span class="hljs-subst">{name}</span>: <span class="hljs-subst">{desc}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L260">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="260">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L261">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="261">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;    <span class="hljs-subst">{model.input_description}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L262">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="262">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">except</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L263">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="263">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;    Input description: <span class="hljs-subst">{<span class="hljs-built_in">type</span>(model.input_description)}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L264">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="264">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-built_in">hasattr</span>(model, <span class="hljs-string">&#x27;output_description&#x27;</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L265">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="265">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;  Outputs:&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L266">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="266">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">try</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L267">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="267">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-keyword">if</span> <span class="hljs-built_in">hasattr</span>(model.output_description, <span class="hljs-string">&#x27;items&#x27;</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L268">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="268">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-keyword">for</span> name, desc <span class="hljs-keyword">in</span> model.output_description.items():<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L269">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="269">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;    <span class="hljs-subst">{name}</span>: <span class="hljs-subst">{desc}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L270">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="270">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L271">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="271">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;    <span class="hljs-subst">{model.output_description}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L272">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="272">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">except</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L273">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="273">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;    Output description: <span class="hljs-subst">{<span class="hljs-built_in">type</span>(model.output_description)}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L274">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="274">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L275">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="275">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L276">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="276">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nWarning: No metadata found in model&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L277">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="277">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L278">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="278">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Check if model directory name contains context length pattern (ctxXXX)</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L279">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="279">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        ctx_len = <span class="hljs-number">512</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L280">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="280">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> args.context_length <span class="hljs-keyword">is</span>  <span class="hljs-literal">None</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L281">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="281">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">import</span> re<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L282">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="282">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            ctx_match = re.search(<span class="hljs-string">r&#x27;ctx(\d+)&#x27;</span>, <span class="hljs-built_in">str</span>(args.d))<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L283">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="283">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> ctx_match:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L284">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="284">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                ctx_len0 = <span class="hljs-built_in">int</span>(ctx_match.group(<span class="hljs-number">1</span>))<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L285">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="285">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">if</span> <span class="hljs-number">512</span> &lt;= ctx_len0 &lt;= <span class="hljs-number">8096</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L286">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="286">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    ctx_len = ctx_len0<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L287">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="287">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nDetected context length <span class="hljs-subst">{ctx_len}</span> from directory name&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L288">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="288">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L289">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="289">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nWarning: No context length found in directory  <span class="hljs-subst">{ctx_len}</span> from directory name <span class="hljs-subst">{args.d}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L290">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="290">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L291">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="291">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            ctx_len = args.context_length<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L292">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="292">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L293">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="293">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Use defaults or values from args</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L294">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="294">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        metadata[<span class="hljs-string">&#x27;context_length&#x27;</span>] = ctx_len<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L295">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="295">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        metadata[<span class="hljs-string">&#x27;state_length&#x27;</span>] = ctx_len<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L296">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="296">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Get batch size from args or use default</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L297">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="297">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        metadata[<span class="hljs-string">&#x27;batch_size&#x27;</span>] = <span class="hljs-built_in">getattr</span>(args, <span class="hljs-string">&#x27;batch_size&#x27;</span>, <span class="hljs-number">64</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L298">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="298">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        metadata[<span class="hljs-string">&#x27;lut_bits&#x27;</span>] = <span class="hljs-number">4</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L299">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="299">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        metadata[<span class="hljs-string">&#x27;num_chunks&#x27;</span>] = <span class="hljs-built_in">getattr</span>(args, <span class="hljs-string">&#x27;num_chunks&#x27;</span>, <span class="hljs-number">4</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L300">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="300">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L301">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="301">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nUsing parameters:&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L302">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="302">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  Context Length: <span class="hljs-subst">{metadata[<span class="hljs-string">&#x27;context_length&#x27;</span>]}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L303">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="303">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  State Length: <span class="hljs-subst">{metadata[<span class="hljs-string">&#x27;state_length&#x27;</span>]}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L304">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="304">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  Prefill Batch Size: <span class="hljs-subst">{metadata[<span class="hljs-string">&#x27;batch_size&#x27;</span>]}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L305">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="305">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  LUT Bits: <span class="hljs-subst">{metadata[<span class="hljs-string">&#x27;lut_bits&#x27;</span>]}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L306">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="306">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  Number of Chunks: <span class="hljs-subst">{metadata[<span class="hljs-string">&#x27;num_chunks&#x27;</span>]}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L307">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="307">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L308">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="308">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Override with values from args if they exist</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L309">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="309">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">if</span> <span class="hljs-built_in">hasattr</span>(args, <span class="hljs-string">&#x27;batch_size&#x27;</span>) <span class="hljs-keyword">and</span> args.batch_size <span class="hljs-keyword">is</span> <span class="hljs-keyword">not</span> <span class="hljs-literal">None</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L310">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="310">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        metadata[<span class="hljs-string">&#x27;batch_size&#x27;</span>] = args.batch_size<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L311">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="311">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L312">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="312">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nOverriding batch size from args: <span class="hljs-subst">{args.batch_size}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L313">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="313">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">if</span> <span class="hljs-built_in">hasattr</span>(args, <span class="hljs-string">&#x27;num_chunks&#x27;</span>) <span class="hljs-keyword">and</span> args.num_chunks <span class="hljs-keyword">is</span> <span class="hljs-keyword">not</span> <span class="hljs-literal">None</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L314">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="314">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        metadata[<span class="hljs-string">&#x27;num_chunks&#x27;</span>] = args.num_chunks<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L315">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="315">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L316">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="316">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nOverriding num chunks from args: <span class="hljs-subst">{args.num_chunks}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L317">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="317">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L318">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="318">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">return</span> metadata<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L319">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="319">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L320">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="320">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">def</span> <span class="hljs-title function_">load_models</span>(<span class="hljs-params">args,metadata</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L321">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="321">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-string">&quot;&quot;&quot;Load all required models and extract metadata.&quot;&quot;&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L322">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="322">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L323">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="323">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nLoading models...&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L324">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="324">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L325">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="325">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">try</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L326">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="326">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Load embeddings model</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L327">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="327">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L328">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="328">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nLoading embeddings model...&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L329">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="329">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        embed_path = parse_model_path(args.embed)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L330">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="330">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L331">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="331">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;Loading from: <span class="hljs-subst">{embed_path}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L332">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="332">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        embed_model = load_model(embed_path)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L333">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="333">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L334">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="334">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;Embeddings model loaded successfully&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L335">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="335">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        metadata = load_metadata(embed_model,args)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L336">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="336">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L337">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="337">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L338">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="338">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L339">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="339">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Load LM head model</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L340">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="340">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L341">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="341">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nLoading LM head model...&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L342">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="342">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        lmhead_path = parse_model_path(args.lmhead)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L343">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="343">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L344">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="344">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;Loading from: <span class="hljs-subst">{lmhead_path}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L345">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="345">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        lmhead_model = load_model(lmhead_path)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L346">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="346">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L347">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="347">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;LM head model loaded successfully&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L348">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="348">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L349">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="349">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Parse FFN path and find chunks if needed</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L350">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="350">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L351">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="351">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nLoading FFN+PREFILL model(s)...&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L352">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="352">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        ffn_path = parse_model_path(args.ffn)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L353">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="353">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        chunk_no, total_chunks = parse_ffn_filename(ffn_path)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L354">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="354">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L355">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="355">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        ffn_models = []<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L356">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="356">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> chunk_no <span class="hljs-keyword">and</span> total_chunks:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L357">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="357">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L358">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="358">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nDetected chunked FFN+PREFILL model (<span class="hljs-subst">{total_chunks}</span> chunks)&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L359">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="359">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-comment"># Find and load all chunks</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L360">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="360">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            chunk_paths = find_all_chunks(ffn_path)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L361">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="361">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-built_in">len</span>(chunk_paths) != total_chunks:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L362">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="362">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">raise</span> ValueError(<span class="hljs-string">f&quot;Found <span class="hljs-subst">{<span class="hljs-built_in">len</span>(chunk_paths)}</span> chunks but filename indicates <span class="hljs-subst">{total_chunks}</span> chunks&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L363">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="363">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L364">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="364">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">for</span> chunk_path <span class="hljs-keyword">in</span> chunk_paths:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L365">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="365">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L366">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="366">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nLoading FFN+PREFILL chunk: <span class="hljs-subst">{Path(chunk_path).name}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L367">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="367">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">try</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L368">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="368">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-comment"># For chunked models, we need both infer and prefill functions</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L369">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="369">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    ffn_models.append({<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L370">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="370">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-string">&#x27;infer&#x27;</span>: load_model(chunk_path, function_name=<span class="hljs-string">&#x27;infer&#x27;</span>),<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L371">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="371">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-string">&#x27;prefill&#x27;</span>: load_model(chunk_path, function_name=<span class="hljs-string">&#x27;prefill&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L372">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="372">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    })<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L373">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="373">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L374">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="374">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;Chunk loaded successfully&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L375">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="375">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">except</span> Exception <span class="hljs-keyword">as</span> e:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L376">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="376">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L377">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="377">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;Error loading chunk <span class="hljs-subst">{chunk_path}</span>: <span class="hljs-subst">{<span class="hljs-built_in">str</span>(e)}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L378">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="378">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-keyword">raise</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L379">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="379">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            metadata = load_metadata(ffn_models[<span class="hljs-number">0</span>],args)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L380">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="380">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L381">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="381">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L382">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="382">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L383">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="383">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nLoading single FFN model...&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L384">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="384">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            ffn_models.append(load_model(ffn_path))<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L385">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="385">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L386">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="386">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;FFN model loaded successfully&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L387">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="387">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L388">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="388">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">return</span> embed_model, ffn_models, lmhead_model, metadata<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L389">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="389">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L390">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="390">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">except</span> Exception <span class="hljs-keyword">as</span> e:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L391">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="391">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nError loading models: <span class="hljs-subst">{<span class="hljs-built_in">str</span>(e)}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L392">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="392">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nPlease ensure all model files exist and are accessible.&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L393">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="393">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;Expected files:&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L394">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="394">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  Embeddings: <span class="hljs-subst">{args.embed}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L395">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="395">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  LM Head: <span class="hljs-subst">{args.lmhead}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L396">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="396">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  FFN: <span class="hljs-subst">{args.ffn}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L397">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="397">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">raise</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L398">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="398">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L399">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="399">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-comment"># At the top of the file, make this a default path</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L400">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="400">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L401">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="401">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">def</span> <span class="hljs-title function_">initialize_tokenizer</span>(<span class="hljs-params">model_path=<span class="hljs-literal">None</span>, eval_mode=<span class="hljs-literal">False</span></span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L402">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="402">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-string">&quot;&quot;&quot;Initialize and configure the tokenizer.&quot;&quot;&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L403">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="403">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">try</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L404">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="404">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L405">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="405">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L406">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="406">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        tokenizer = AutoTokenizer.from_pretrained(<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L407">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="407">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">str</span>(model_path), <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L408">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="408">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            use_fast=<span class="hljs-literal">False</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L409">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="409">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            trust_remote_code=<span class="hljs-literal">True</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L410">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="410">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        )<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L411">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="411">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L412">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="412">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> eval_mode:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L413">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="413">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nTokenizer Configuration:&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L414">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="414">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;Tokenizer type: <span class="hljs-subst">{<span class="hljs-built_in">type</span>(tokenizer)}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L415">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="415">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;Tokenizer name: <span class="hljs-subst">{tokenizer.__class__.__name__}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L416">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="416">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;Vocabulary size: <span class="hljs-subst">{<span class="hljs-built_in">len</span>(tokenizer)}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L417">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="417">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;Model max length: <span class="hljs-subst">{tokenizer.model_max_length}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L418">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="418">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L419">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="419">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> tokenizer.pad_token <span class="hljs-keyword">is</span> <span class="hljs-literal">None</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L420">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="420">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            tokenizer.pad_token = tokenizer.eos_token<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L421">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="421">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            tokenizer.pad_token_id = tokenizer.eos_token_id<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L422">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="422">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> eval_mode:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L423">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="423">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;Set PAD token to EOS token&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L424">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="424">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L425">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="425">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        tokenizer.padding_side = <span class="hljs-string">&quot;left&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L426">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="426">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L427">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="427">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> eval_mode:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L428">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="428">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nSpecial Tokens:&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L429">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="429">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;PAD token: &#x27;<span class="hljs-subst">{tokenizer.pad_token}</span>&#x27; (ID: <span class="hljs-subst">{tokenizer.pad_token_id}</span>)&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L430">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="430">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;EOS token: &#x27;<span class="hljs-subst">{tokenizer.eos_token}</span>&#x27; (ID: <span class="hljs-subst">{tokenizer.eos_token_id}</span>)&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L431">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="431">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;BOS token: &#x27;<span class="hljs-subst">{tokenizer.bos_token}</span>&#x27; (ID: <span class="hljs-subst">{tokenizer.bos_token_id}</span>)&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L432">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="432">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;UNK token: &#x27;<span class="hljs-subst">{tokenizer.unk_token}</span>&#x27; (ID: <span class="hljs-subst">{tokenizer.unk_token_id}</span>)&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L433">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="433">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L434">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="434">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">return</span> tokenizer<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L435">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="435">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L436">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="436">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">except</span> Exception <span class="hljs-keyword">as</span> e:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L437">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="437">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nError: Failed to load tokenizer from <span class="hljs-subst">{model_path}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L438">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="438">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;Error details: <span class="hljs-subst">{<span class="hljs-built_in">str</span>(e)}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L439">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="439">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;Error type: <span class="hljs-subst">{<span class="hljs-built_in">type</span>(e)}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L440">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="440">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nThis appears to be a tokenizer loading issue.&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L441">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="441">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L442">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="442">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Check if it&#x27;s the specific Qwen tokenizer file issue</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L443">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="443">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-string">&quot;expected str, bytes or os.PathLike object, not NoneType&quot;</span> <span class="hljs-keyword">in</span> <span class="hljs-built_in">str</span>(e):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L444">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="444">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nThis error suggests the tokenizer files are missing or incomplete.&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L445">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="445">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;For Qwen models, you need the original model directory with tokenizer files.&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L446">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="446">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;Try using: --tokenizer ~/.cache/huggingface/hub/models--Qwen--Qwen3-0.6B/snapshots/YOUR_SNAPSHOT_ID&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L447">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="447">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L448">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="448">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;Please provide the path to a compatible model directory with tokenizer files.&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L449">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="449">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">import</span> traceback<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L450">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="450">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        traceback.print_exc()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L451">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="451">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">raise</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L452">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="452">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L453">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="453">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L454">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="454">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L455">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="455">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">def</span> <span class="hljs-title function_">make_causal_mask</span>(<span class="hljs-params">length, start</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L456">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="456">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-string">&quot;&quot;&quot;Create causal attention mask.&quot;&quot;&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L457">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="457">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    mask = np.full((<span class="hljs-number">1</span>, <span class="hljs-number">1</span>, length, length), -np.inf, dtype=np.float16)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L458">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="458">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    row_indices = np.arange(length).reshape(length, <span class="hljs-number">1</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L459">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="459">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    col_indices = np.arange(length).reshape(<span class="hljs-number">1</span>, length)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L460">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="460">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    mask[:, :, col_indices &lt;= (row_indices + start)] = <span class="hljs-number">0</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L461">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="461">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">return</span> mask<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L462">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="462">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L463">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="463">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">def</span> <span class="hljs-title function_">initialize_causal_mask</span>(<span class="hljs-params">context_length, eval_mode=<span class="hljs-literal">False</span></span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L464">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="464">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-string">&quot;&quot;&quot;Initialize causal mask for transformer attention.&quot;&quot;&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L465">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="465">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    causal_mask = make_causal_mask(context_length, <span class="hljs-number">0</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L466">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="466">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    causal_mask = torch.tensor(causal_mask, dtype=torch.float16)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L467">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="467">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> eval_mode:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L468">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="468">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nInitialized causal mask for context length <span class="hljs-subst">{context_length}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L469">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="469">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">return</span> causal_mask<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L470">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="470">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L471">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="471">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">def</span> <span class="hljs-title function_">run_prefill</span>(<span class="hljs-params">embed_model, ffn_models, input_ids, context_pos, context_length, batch_size=<span class="hljs-number">64</span>, state=<span class="hljs-literal">None</span>, causal_mask=<span class="hljs-literal">None</span></span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L472">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="472">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-string">&quot;&quot;&quot;Run prefill on the input sequence.&quot;&quot;&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L473">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="473">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Use provided causal mask or create one if not provided</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L474">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="474">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">if</span> causal_mask <span class="hljs-keyword">is</span> <span class="hljs-literal">None</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L475">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="475">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        causal_mask = make_causal_mask(context_length, <span class="hljs-number">0</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L476">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="476">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        causal_mask = torch.tensor(causal_mask, dtype=torch.float16)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L477">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="477">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L478">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="478">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Process in batches</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L479">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="479">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    batch_pos = <span class="hljs-number">0</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L480">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="480">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">while</span> batch_pos &lt; context_pos:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L481">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="481">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        batch_end = <span class="hljs-built_in">min</span>(batch_pos + batch_size, context_pos)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L482">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="482">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        current_batch_size = batch_end - batch_pos<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L483">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="483">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L484">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="484">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Get current batch</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L485">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="485">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        batch_input = input_ids[:, batch_pos:batch_end]<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L486">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="486">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L487">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="487">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Always pad to full batch size for prefill</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L488">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="488">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        batch_input = F.pad(<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L489">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="489">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            batch_input,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L490">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="490">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            (<span class="hljs-number">0</span>, batch_size - current_batch_size),<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L491">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="491">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            value=<span class="hljs-number">0</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L492">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="492">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        )<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L493">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="493">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L494">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="494">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Generate position IDs for full batch size</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L495">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="495">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        position_ids = torch.arange(batch_pos, batch_pos+batch_size, dtype=torch.int32)  <span class="hljs-comment"># Changed: Always use full batch size</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L496">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="496">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        batch_causal_mask = causal_mask[:, :, batch_pos:batch_pos+batch_size, :]  <span class="hljs-comment"># Changed: Use full batch size</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L497">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="497">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L498">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="498">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Run embeddings</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L499">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="499">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        hidden_states = torch.from_numpy(<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L500">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="500">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            embed_model.predict({<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L501">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="501">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-string">&#x27;input_ids&#x27;</span>: batch_input.numpy().astype(np.int32)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L502">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="502">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            })[<span class="hljs-string">&#x27;hidden_states&#x27;</span>]<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L503">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="503">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        )<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L504">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="504">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L505">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="505">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Run through FFN chunks with state</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L506">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="506">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">for</span> ffn_model <span class="hljs-keyword">in</span> ffn_models:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L507">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="507">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-built_in">isinstance</span>(ffn_model, <span class="hljs-built_in">dict</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L508">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="508">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                inputs = {<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L509">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="509">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-string">&#x27;hidden_states&#x27;</span>: hidden_states.numpy().astype(np.float16),  <span class="hljs-comment"># [1, 64, hidden_size]</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L510">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="510">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-string">&#x27;position_ids&#x27;</span>: position_ids.numpy().astype(np.int32),    <span class="hljs-comment"># [64]</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L511">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="511">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-string">&#x27;causal_mask&#x27;</span>: batch_causal_mask.numpy().astype(np.float16), <span class="hljs-comment"># [1, 1, 64, context_length]</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L512">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="512">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-string">&#x27;current_pos&#x27;</span>: np.array([batch_pos], dtype=np.int32)  <span class="hljs-comment"># [1]</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L513">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="513">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                }<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L514">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="514">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                output = ffn_model[<span class="hljs-string">&#x27;prefill&#x27;</span>].predict(inputs, state)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L515">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="515">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                hidden_states = torch.from_numpy(output[<span class="hljs-string">&#x27;output_hidden_states&#x27;</span>])<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L516">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="516">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L517">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="517">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        batch_pos = batch_end<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L518">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="518">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L519">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="519">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">return</span> torch.tensor([context_pos], dtype=torch.int32)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L520">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="520">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L521">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="521">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">def</span> <span class="hljs-title function_">generate_next_token</span>(<span class="hljs-params">embed_model, ffn_models, lmhead_model, input_ids, pos, context_length, metadata, state=<span class="hljs-literal">None</span>, causal_mask=<span class="hljs-literal">None</span>, temperature=<span class="hljs-number">0.0</span></span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L522">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="522">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-string">&quot;&quot;&quot;Generate the next token.&quot;&quot;&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L523">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="523">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Get current token</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L524">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="524">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    current_token = input_ids[:, pos-<span class="hljs-number">1</span>:pos]  <span class="hljs-comment"># [1, 1]</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L525">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="525">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L526">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="526">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Ensure proper data type for CoreML</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L527">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="527">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    current_token_array = current_token.numpy().astype(np.int32)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L528">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="528">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L529">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="529">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Run embeddings</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L530">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="530">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    hidden_states = torch.from_numpy(<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L531">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="531">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        embed_model.predict({<span class="hljs-string">&#x27;input_ids&#x27;</span>: current_token_array})[<span class="hljs-string">&#x27;hidden_states&#x27;</span>]<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L532">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="532">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    )  <span class="hljs-comment"># [1, 1, hidden_size]</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L533">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="533">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L534">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="534">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Create masks</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L535">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="535">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    update_mask = torch.zeros((<span class="hljs-number">1</span>, <span class="hljs-number">1</span>, context_length, <span class="hljs-number">1</span>), dtype=torch.float16)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L536">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="536">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    update_mask[<span class="hljs-number">0</span>, <span class="hljs-number">0</span>, pos-<span class="hljs-number">1</span>, <span class="hljs-number">0</span>] = <span class="hljs-number">1.0</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L537">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="537">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    position_ids = torch.tensor([pos-<span class="hljs-number">1</span>], dtype=torch.int32)  <span class="hljs-comment"># [1]</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L538">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="538">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L539">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="539">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Use provided causal mask or create one if not provided</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L540">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="540">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">if</span> causal_mask <span class="hljs-keyword">is</span> <span class="hljs-literal">None</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L541">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="541">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        causal_mask_data = make_causal_mask(context_length, <span class="hljs-number">0</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L542">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="542">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        single_causal_mask = torch.tensor(causal_mask_data[:, :, pos-<span class="hljs-number">1</span>:pos, :], dtype=torch.float16)  <span class="hljs-comment"># [1, 1, 1, context_length]</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L543">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="543">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L544">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="544">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        single_causal_mask = causal_mask[:, :, pos-<span class="hljs-number">1</span>:pos, :]<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L545">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="545">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L546">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="546">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Run through FFN chunks with state</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L547">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="547">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">for</span> ffn_model <span class="hljs-keyword">in</span> ffn_models:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L548">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="548">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-built_in">isinstance</span>(ffn_model, <span class="hljs-built_in">dict</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L549">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="549">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            inputs = {<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L550">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="550">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-string">&#x27;hidden_states&#x27;</span>: hidden_states.numpy().astype(np.float16),<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L551">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="551">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-string">&#x27;update_mask&#x27;</span>: update_mask.numpy().astype(np.float16),<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L552">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="552">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-string">&#x27;position_ids&#x27;</span>: position_ids.numpy().astype(np.int32),<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L553">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="553">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-string">&#x27;causal_mask&#x27;</span>: single_causal_mask.numpy().astype(np.float16),<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L554">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="554">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-string">&#x27;current_pos&#x27;</span>: position_ids.numpy().astype(np.int32)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L555">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="555">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            }<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L556">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="556">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            output = ffn_model[<span class="hljs-string">&#x27;infer&#x27;</span>].predict(inputs, state)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L557">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="557">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            hidden_states = torch.from_numpy(output[<span class="hljs-string">&#x27;output_hidden_states&#x27;</span>])<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L558">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="558">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L559">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="559">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Run LM head</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L560">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="560">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    lm_output = lmhead_model.predict({<span class="hljs-string">&#x27;hidden_states&#x27;</span>: hidden_states.numpy().astype(np.float16)})<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L561">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="561">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Debug print</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L562">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="562">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment">#print(&quot;\nLM Head output keys:&quot;, list(lm_output.keys()))</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L563">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="563">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L564">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="564">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Get number of logits from metadata, using split_lm_head if available</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L565">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="565">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># First check for split_lm_head (new), then num_logits (legacy), default to 8</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L566">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="566">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    num_logits = metadata.get(<span class="hljs-string">&#x27;split_lm_head&#x27;</span>, metadata.get(<span class="hljs-string">&#x27;num_logits&#x27;</span>, <span class="hljs-number">8</span>))<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L567">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="567">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L568">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="568">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Combine logits1-N if they exist</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L569">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="569">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">if</span> <span class="hljs-string">&#x27;logits1&#x27;</span> <span class="hljs-keyword">in</span> lm_output:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L570">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="570">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Concatenate all logits parts</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L571">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="571">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        logits_parts = []<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L572">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="572">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">for</span> i <span class="hljs-keyword">in</span> <span class="hljs-built_in">range</span>(<span class="hljs-number">1</span>, num_logits + <span class="hljs-number">1</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L573">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="573">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            key = <span class="hljs-string">f&#x27;logits<span class="hljs-subst">{i}</span>&#x27;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L574">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="574">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> key <span class="hljs-keyword">in</span> lm_output:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L575">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="575">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                logits_parts.append(torch.from_numpy(lm_output[key]))<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L576">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="576">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        logits = torch.cat(logits_parts, dim=-<span class="hljs-number">1</span>)  <span class="hljs-comment"># Concatenate along vocab dimension</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L577">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="577">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L578">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="578">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Try output_logits as fallback</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L579">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="579">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        logits = torch.from_numpy(lm_output[<span class="hljs-string">&#x27;output_logits&#x27;</span>])<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L580">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="580">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L581">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="581">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Apply temperature and sample</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L582">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="582">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">if</span> temperature &gt; <span class="hljs-number">0</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L583">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="583">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        logits = logits / temperature<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L584">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="584">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        probs = F.softmax(logits[<span class="hljs-number">0</span>, -<span class="hljs-number">1</span>, :], dim=-<span class="hljs-number">1</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L585">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="585">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        next_token = torch.multinomial(probs, num_samples=<span class="hljs-number">1</span>).item()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L586">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="586">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L587">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="587">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        next_token = torch.argmax(logits[<span class="hljs-number">0</span>, -<span class="hljs-number">1</span>, :]).item()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L588">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="588">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L589">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="589">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">return</span> next_token<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L590">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="590">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L591">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="591">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">def</span> <span class="hljs-title function_">create_unified_state</span>(<span class="hljs-params">ffn_models, context_length, eval_mode=<span class="hljs-literal">False</span></span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L592">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="592">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-string">&quot;&quot;&quot;Create unified KV cache state for transformer.&quot;&quot;&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L593">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="593">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">if</span> <span class="hljs-built_in">isinstance</span>(ffn_models[<span class="hljs-number">0</span>], <span class="hljs-built_in">dict</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L594">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="594">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Use first FFN model&#x27;s prefill function to create state</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L595">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="595">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        state = ffn_models[<span class="hljs-number">0</span>][<span class="hljs-string">&#x27;prefill&#x27;</span>].make_state()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L596">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="596">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> eval_mode:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L597">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="597">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nCreated unified transformer state for <span class="hljs-subst">{<span class="hljs-built_in">len</span>(ffn_models)}</span> chunks&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L598">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="598">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">return</span> state<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L599">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="599">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L600">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="600">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        state = ffn_models[<span class="hljs-number">0</span>].make_state()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L601">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="601">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> eval_mode:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L602">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="602">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nCreated unified transformer state&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L603">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="603">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">return</span> state<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L604">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="604">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L605">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="605">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">def</span> <span class="hljs-title function_">chat_loop</span>(<span class="hljs-params">embed_model, ffn_models, lmhead_model, tokenizer, metadata, state, causal_mask=<span class="hljs-literal">None</span>, auto_prompt=<span class="hljs-literal">None</span>, warmup=<span class="hljs-literal">False</span>, save_file=<span class="hljs-literal">None</span>, max_tokens=<span class="hljs-literal">None</span>, no_template=<span class="hljs-literal">False</span>, eval_mode=<span class="hljs-literal">False</span></span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L606">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="606">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-string">&quot;&quot;&quot;Interactive chat loop.&quot;&quot;&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L607">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="607">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    context_length = metadata.get(<span class="hljs-string">&#x27;context_length&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L608">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="608">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    batch_size = metadata.get(<span class="hljs-string">&#x27;batch_size&#x27;</span>, <span class="hljs-number">64</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L609">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="609">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L610">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="610">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> warmup <span class="hljs-keyword">and</span> <span class="hljs-keyword">not</span> eval_mode:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L611">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="611">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nUsing context length: <span class="hljs-subst">{context_length}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L612">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="612">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nStarting chat session. Press Ctrl+D to exit.&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L613">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="613">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;Type your message and press Enter to chat.&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L614">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="614">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L615">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="615">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Check if tokenizer has chat template and if it works</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L616">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="616">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    has_chat_template = <span class="hljs-literal">False</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L617">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="617">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">try</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L618">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="618">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Test if chat template works</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L619">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="619">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        test_messages = [{<span class="hljs-string">&quot;role&quot;</span>: <span class="hljs-string">&quot;user&quot;</span>, <span class="hljs-string">&quot;content&quot;</span>: <span class="hljs-string">&quot;test&quot;</span>}]<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L620">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="620">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        tokenizer.apply_chat_template(test_messages, return_tensors=<span class="hljs-string">&quot;pt&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L621">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="621">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        has_chat_template = <span class="hljs-literal">True</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L622">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="622">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> warmup <span class="hljs-keyword">and</span> <span class="hljs-keyword">not</span> eval_mode:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L623">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="623">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nUsing chat template for prompts&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L624">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="624">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">except</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L625">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="625">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> warmup <span class="hljs-keyword">and</span> <span class="hljs-keyword">not</span> eval_mode:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L626">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="626">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nUsing manual formatting for prompts&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L627">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="627">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L628">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="628">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    conversation = []<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L629">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="629">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L630">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="630">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">try</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L631">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="631">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">while</span> <span class="hljs-literal">True</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L632">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="632">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">try</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L633">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="633">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> warmup <span class="hljs-keyword">and</span> <span class="hljs-keyword">not</span> eval_mode:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L634">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="634">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\n<span class="hljs-subst">{LIGHT_GREEN}</span>You:<span class="hljs-subst">{RESET_COLOR}</span>&quot;</span>, end=<span class="hljs-string">&#x27; &#x27;</span>, flush=<span class="hljs-literal">True</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L635">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="635">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">if</span> auto_prompt <span class="hljs-keyword">is</span> <span class="hljs-keyword">not</span> <span class="hljs-literal">None</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L636">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="636">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    user_input = auto_prompt<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L637">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="637">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> warmup <span class="hljs-keyword">and</span> <span class="hljs-keyword">not</span> eval_mode:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L638">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="638">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-built_in">print</span>(user_input)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L639">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="639">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L640">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="640">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    user_input = <span class="hljs-built_in">input</span>().strip()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L641">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="641">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">except</span> EOFError:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L642">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="642">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> warmup <span class="hljs-keyword">and</span> <span class="hljs-keyword">not</span> eval_mode:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L643">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="643">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nExiting chat...&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L644">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="644">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">break</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L645">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="645">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L646">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="646">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> user_input:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L647">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="647">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">continue</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L648">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="648">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L649">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="649">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-comment"># Format prompt based on no_template flag and tokenizer capabilities</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L650">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="650">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> no_template:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L651">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="651">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-comment"># Use raw input without any chat template formatting</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L652">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="652">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                input_ids = tokenizer(<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L653">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="653">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    user_input,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L654">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="654">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    return_tensors=<span class="hljs-string">&quot;pt&quot;</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L655">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="655">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    add_special_tokens=<span class="hljs-literal">True</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L656">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="656">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                ).input_ids.to(torch.int32)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L657">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="657">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> warmup <span class="hljs-keyword">and</span> <span class="hljs-keyword">not</span> eval_mode:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L658">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="658">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;Using raw input without chat template&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L659">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="659">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">elif</span> has_chat_template:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L660">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="660">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                messages = [{<span class="hljs-string">&quot;role&quot;</span>: <span class="hljs-string">&quot;user&quot;</span>, <span class="hljs-string">&quot;content&quot;</span>: user_input}]<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L661">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="661">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                input_ids = tokenizer.apply_chat_template(<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L662">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="662">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    messages,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L663">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="663">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    return_tensors=<span class="hljs-string">&quot;pt&quot;</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L664">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="664">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    add_generation_prompt=<span class="hljs-literal">True</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L665">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="665">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                ).to(torch.int32)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L666">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="666">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L667">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="667">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-comment"># Manual formatting for Llama models without chat template</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L668">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="668">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                formatted_prompt = <span class="hljs-string">f&quot;[INST] <span class="hljs-subst">{user_input}</span> [/INST]&quot;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L669">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="669">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                input_ids = tokenizer(<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L670">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="670">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    formatted_prompt,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L671">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="671">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    return_tensors=<span class="hljs-string">&quot;pt&quot;</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L672">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="672">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    add_special_tokens=<span class="hljs-literal">True</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L673">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="673">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                ).input_ids.to(torch.int32)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L674">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="674">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L675">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="675">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            context_pos = input_ids.size(<span class="hljs-number">1</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L676">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="676">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L677">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="677">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> warmup <span class="hljs-keyword">and</span> <span class="hljs-keyword">not</span> eval_mode:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L678">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="678">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\n<span class="hljs-subst">{LIGHT_BLUE}</span>Assistant:<span class="hljs-subst">{RESET_COLOR}</span>&quot;</span>, end=<span class="hljs-string">&#x27; &#x27;</span>, flush=<span class="hljs-literal">True</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L679">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="679">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L680">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="680">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-comment"># Initialize token printer</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L681">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="681">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            token_printer = TokenPrinter(tokenizer)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L682">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="682">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            tokens_generated = <span class="hljs-number">0</span>  <span class="hljs-comment"># Track number of tokens</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L683">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="683">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L684">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="684">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">try</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L685">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="685">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-comment"># Start prefill timing</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L686">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="686">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                prefill_start = time.time()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L687">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="687">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L688">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="688">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-comment"># Run prefill with state and causal mask</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L689">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="689">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-comment"># Ensure batch_size is not None</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L690">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="690">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">if</span> batch_size <span class="hljs-keyword">is</span> <span class="hljs-literal">None</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L691">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="691">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    batch_size = <span class="hljs-number">64</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L692">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="692">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> eval_mode:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L693">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="693">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;Warning: batch_size was None, using default: <span class="hljs-subst">{batch_size}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L694">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="694">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L695">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="695">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                _ = run_prefill(<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L696">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="696">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    embed_model,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L697">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="697">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    ffn_models,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L698">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="698">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    input_ids,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L699">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="699">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    context_pos,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L700">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="700">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    context_length,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L701">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="701">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    batch_size,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L702">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="702">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    state,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L703">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="703">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    causal_mask<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L704">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="704">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                )<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L705">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="705">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L706">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="706">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-comment"># Calculate prefill timing</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L707">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="707">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                prefill_time = time.time() - prefill_start<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L708">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="708">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                prefill_tokens = context_pos  <span class="hljs-comment"># Number of tokens in input</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L709">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="709">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                prefill_tokens_per_sec = prefill_tokens / prefill_time <span class="hljs-keyword">if</span> prefill_time &gt; <span class="hljs-number">0</span> <span class="hljs-keyword">else</span> <span class="hljs-number">0</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L710">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="710">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L711">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="711">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-comment"># Generation loop with state</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L712">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="712">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                input_ids = input_ids<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L713">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="713">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                pos = context_pos<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L714">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="714">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                inference_start = time.time()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L715">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="715">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                inference_tokens = <span class="hljs-number">0</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L716">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="716">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L717">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="717">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">while</span> pos &lt; context_length - <span class="hljs-number">1</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L718">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="718">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-comment"># Generate next token with causal mask</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L719">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="719">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    next_token = generate_next_token(<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L720">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="720">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        embed_model,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L721">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="721">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        ffn_models,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L722">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="722">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        lmhead_model,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L723">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="723">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        input_ids,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L724">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="724">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        pos,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L725">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="725">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        context_length,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L726">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="726">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        metadata,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L727">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="727">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        state,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L728">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="728">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        causal_mask<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L729">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="729">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    )<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L730">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="730">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L731">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="731">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-comment"># Add token to sequence</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L732">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="732">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-keyword">if</span> pos &lt; input_ids.size(<span class="hljs-number">1</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L733">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="733">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        input_ids[<span class="hljs-number">0</span>, pos] = next_token<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L734">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="734">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L735">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="735">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        input_ids = torch.cat([<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L736">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="736">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                            input_ids,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L737">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="737">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                            torch.tensor([[next_token]], dtype=torch.int32)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L738">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="738">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        ], dim=<span class="hljs-number">1</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L739">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="739">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L740">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="740">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-comment"># Add to printer only if not in warmup</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L741">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="741">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> warmup:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L742">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="742">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        token_printer.add_token(next_token)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L743">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="743">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        token_printer.drain_buffer(eval_mode)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L744">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="744">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L745">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="745">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    pos += <span class="hljs-number">1</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L746">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="746">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    tokens_generated += <span class="hljs-number">1</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L747">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="747">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    inference_tokens += <span class="hljs-number">1</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L748">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="748">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L749">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="749">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-comment"># Check limits</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L750">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="750">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-keyword">if</span> warmup <span class="hljs-keyword">and</span> tokens_generated &gt;= WARMUP_TOKEN_LIMIT:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L751">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="751">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-keyword">break</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L752">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="752">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L753">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="753">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-comment"># Check max_tokens limit</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L754">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="754">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-keyword">if</span> max_tokens <span class="hljs-keyword">is</span> <span class="hljs-keyword">not</span> <span class="hljs-literal">None</span> <span class="hljs-keyword">and</span> tokens_generated &gt;= max_tokens:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L755">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="755">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-keyword">break</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L756">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="756">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L757">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="757">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-comment"># Check for all possible EOS tokens</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L758">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="758">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    eos_token_ids = tokenizer.eos_token_id<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L759">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="759">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-keyword">if</span> <span class="hljs-built_in">isinstance</span>(eos_token_ids, <span class="hljs-built_in">list</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L760">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="760">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-keyword">if</span> next_token <span class="hljs-keyword">in</span> eos_token_ids:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L761">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="761">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                            <span class="hljs-keyword">break</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L762">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="762">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L763">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="763">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-keyword">if</span> next_token == eos_token_ids:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L764">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="764">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                            <span class="hljs-keyword">break</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L765">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="765">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L766">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="766">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-comment"># Calculate inference timing</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L767">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="767">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                inference_time = time.time() - inference_start<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L768">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="768">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                inference_tokens_per_sec = inference_tokens / inference_time <span class="hljs-keyword">if</span> inference_time &gt; <span class="hljs-number">0</span> <span class="hljs-keyword">else</span> <span class="hljs-number">0</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L769">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="769">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L770">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="770">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-comment"># Get final response and add to conversation</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L771">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="771">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> warmup:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L772">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="772">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    response = token_printer.stop(eval_mode)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L773">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="773">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-keyword">if</span> eval_mode:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L774">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="774">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-comment"># In eval mode, only print the model response</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L775">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="775">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-built_in">print</span>(response, end=<span class="hljs-string">&#x27;&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L776">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="776">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L777">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="777">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-comment"># Print timing stats</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L778">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="778">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        prefill_ms = prefill_time * <span class="hljs-number">1000</span>  <span class="hljs-comment"># Convert to milliseconds</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L779">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="779">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nPrefill: <span class="hljs-subst">{prefill_ms:<span class="hljs-number">.1</span>f}</span>ms (<span class="hljs-subst">{prefill_tokens_per_sec:<span class="hljs-number">.1</span>f}</span> t/s)&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L780">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="780">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;Inference: <span class="hljs-subst">{inference_tokens_per_sec:<span class="hljs-number">.1</span>f}</span> t/s&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L781">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="781">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;Total: Generated <span class="hljs-subst">{tokens_generated}</span> tokens in <span class="hljs-subst">{prefill_time + inference_time:<span class="hljs-number">.2</span>f}</span>s&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L782">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="782">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    conversation.append({<span class="hljs-string">&quot;role&quot;</span>: <span class="hljs-string">&quot;assistant&quot;</span>, <span class="hljs-string">&quot;content&quot;</span>: response})<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L783">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="783">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L784">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="784">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-comment"># Save response to file if requested</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L785">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="785">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-keyword">if</span> save_file <span class="hljs-keyword">and</span> <span class="hljs-keyword">not</span> eval_mode:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L786">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="786">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-keyword">try</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L787">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="787">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                            <span class="hljs-comment"># Add small delay to ensure all tokens are processed</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L788">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="788">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                            time.sleep(<span class="hljs-number">0.5</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L789">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="789">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                            <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L790">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="790">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                            <span class="hljs-comment"># Make sure response ends with EOS token if it&#x27;s supposed to</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L791">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="791">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                            <span class="hljs-keyword">if</span> response <span class="hljs-keyword">and</span> <span class="hljs-keyword">not</span> response.endswith(<span class="hljs-string">&quot;&lt;|eot_id|&gt;&quot;</span>) <span class="hljs-keyword">and</span> <span class="hljs-keyword">not</span> response.endswith(<span class="hljs-string">&quot;&lt;/s&gt;&quot;</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L792">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="792">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                                <span class="hljs-keyword">if</span> tokenizer.eos_token:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L793">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="793">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                                    eos_text = tokenizer.decode([tokenizer.eos_token_id])<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L794">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="794">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                                    <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> response.endswith(eos_text):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L795">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="795">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                                        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\n<span class="hljs-subst">{DARK_BLUE}</span>Adding missing EOS token for consistency<span class="hljs-subst">{RESET_COLOR}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L796">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="796">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                                        response += eos_text<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L797">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="797">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                            <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L798">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="798">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                            <span class="hljs-keyword">with</span> <span class="hljs-built_in">open</span>(save_file, <span class="hljs-string">&#x27;w&#x27;</span>) <span class="hljs-keyword">as</span> f:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L799">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="799">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                                f.write(response)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L800">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="800">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\n<span class="hljs-subst">{DARK_BLUE}</span>Response saved to file: <span class="hljs-subst">{save_file}</span><span class="hljs-subst">{RESET_COLOR}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L801">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="801">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                        <span class="hljs-keyword">except</span> Exception <span class="hljs-keyword">as</span> e:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L802">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="802">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\n<span class="hljs-subst">{DARK_BLUE}</span>Error saving to file: <span class="hljs-subst">{<span class="hljs-built_in">str</span>(e)}</span><span class="hljs-subst">{RESET_COLOR}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L803">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="803">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L804">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="804">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    token_printer.stop(eval_mode)  <span class="hljs-comment"># Clean up without printing stats</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L805">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="805">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L806">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="806">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-comment"># Exit after one response in auto_prompt mode</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L807">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="807">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">if</span> auto_prompt <span class="hljs-keyword">is</span> <span class="hljs-keyword">not</span> <span class="hljs-literal">None</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L808">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="808">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-keyword">break</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L809">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="809">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L810">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="810">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">except</span> KeyboardInterrupt:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L811">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="811">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> eval_mode:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L812">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="812">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nGeneration interrupted&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L813">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="813">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                token_printer.stop(eval_mode)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L814">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="814">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">continue</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L815">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="815">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L816">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="816">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">except</span> Exception <span class="hljs-keyword">as</span> e:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L817">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="817">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nError in chat loop: <span class="hljs-subst">{<span class="hljs-built_in">str</span>(e)}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L818">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="818">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">import</span> traceback<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L819">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="819">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        traceback.print_exc()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L820">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="820">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L821">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="821">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">def</span> <span class="hljs-title function_">parse_args</span>():<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L822">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="822">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    parser = argparse.ArgumentParser(description=<span class="hljs-string">&#x27;Chat with CoreML LLaMA, gil resolved  (c) 2025 Anemll&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L823">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="823">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L824">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="824">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Add meta.yaml option</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L825">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="825">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    parser.add_argument(<span class="hljs-string">&#x27;--meta&#x27;</span>, <span class="hljs-built_in">type</span>=<span class="hljs-built_in">str</span>, <span class="hljs-built_in">help</span>=<span class="hljs-string">&#x27;Path to meta.yaml to load all parameters&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L826">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="826">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L827">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="827">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Model paths</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L828">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="828">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    parser.add_argument(<span class="hljs-string">&#x27;--d&#x27;</span>, <span class="hljs-string">&#x27;--dir&#x27;</span>, <span class="hljs-built_in">type</span>=<span class="hljs-built_in">str</span>, default=<span class="hljs-string">&#x27;.&#x27;</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L829">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="829">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                       <span class="hljs-built_in">help</span>=<span class="hljs-string">&#x27;Directory containing model files (default: current directory)&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L830">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="830">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    parser.add_argument(<span class="hljs-string">&#x27;--embed&#x27;</span>, <span class="hljs-built_in">type</span>=<span class="hljs-built_in">str</span>, required=<span class="hljs-literal">False</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L831">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="831">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                       <span class="hljs-built_in">help</span>=<span class="hljs-string">&#x27;Path to embeddings model (relative to --dir)&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L832">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="832">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    parser.add_argument(<span class="hljs-string">&#x27;--ffn&#x27;</span>, <span class="hljs-built_in">type</span>=<span class="hljs-built_in">str</span>, required=<span class="hljs-literal">False</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L833">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="833">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                       <span class="hljs-built_in">help</span>=<span class="hljs-string">&#x27;Path to FFN model (can be chunked, relative to --dir)&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L834">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="834">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    parser.add_argument(<span class="hljs-string">&#x27;--lmhead&#x27;</span>, <span class="hljs-built_in">type</span>=<span class="hljs-built_in">str</span>, required=<span class="hljs-literal">False</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L835">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="835">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                       <span class="hljs-built_in">help</span>=<span class="hljs-string">&#x27;Path to LM head model (relative to --dir)&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L836">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="836">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    parser.add_argument(<span class="hljs-string">&#x27;--tokenizer&#x27;</span>, <span class="hljs-built_in">type</span>=<span class="hljs-built_in">str</span>, required=<span class="hljs-literal">False</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L837">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="837">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                       <span class="hljs-built_in">help</span>=<span class="hljs-string">&#x27;Path to tokenizer&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L838">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="838">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L839">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="839">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Add new argument for auto-generation</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L840">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="840">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    parser.add_argument(<span class="hljs-string">&#x27;--prompt&#x27;</span>, <span class="hljs-built_in">type</span>=<span class="hljs-built_in">str</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L841">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="841">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                       <span class="hljs-built_in">help</span>=<span class="hljs-string">&#x27;If specified, run once with this prompt and exit&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L842">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="842">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L843">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="843">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Add save option</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L844">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="844">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    parser.add_argument(<span class="hljs-string">&#x27;--save&#x27;</span>, <span class="hljs-built_in">type</span>=<span class="hljs-built_in">str</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L845">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="845">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                       <span class="hljs-built_in">help</span>=<span class="hljs-string">&#x27;Save assistant\&#x27;s response to specified file&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L846">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="846">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L847">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="847">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Add max-tokens option</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L848">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="848">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    parser.add_argument(<span class="hljs-string">&#x27;--max-tokens&#x27;</span>, <span class="hljs-built_in">type</span>=<span class="hljs-built_in">int</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L849">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="849">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                       <span class="hljs-built_in">help</span>=<span class="hljs-string">&#x27;Maximum number of tokens to generate&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L850">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="850">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L851">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="851">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Add no-warmup flag</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L852">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="852">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    parser.add_argument(<span class="hljs-string">&#x27;--nw&#x27;</span>, action=<span class="hljs-string">&#x27;store_true&#x27;</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L853">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="853">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                       <span class="hljs-built_in">help</span>=<span class="hljs-string">&#x27;Skip warmup phase&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L854">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="854">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L855">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="855">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Add no-template flag  </span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L856">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="856">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    parser.add_argument(<span class="hljs-string">&#x27;--no-template&#x27;</span>, action=<span class="hljs-string">&#x27;store_true&#x27;</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L857">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="857">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                       <span class="hljs-built_in">help</span>=<span class="hljs-string">&#x27;Prefill the question itself and start inference directly without chat template&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L858">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="858">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L859">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="859">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Add eval mode flag</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L860">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="860">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    parser.add_argument(<span class="hljs-string">&#x27;--eval&#x27;</span>, action=<span class="hljs-string">&#x27;store_true&#x27;</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L861">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="861">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                       <span class="hljs-built_in">help</span>=<span class="hljs-string">&#x27;Evaluation mode: suppress all output except model response&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L862">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="862">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L863">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="863">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Model configuration</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L864">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="864">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    parser.add_argument(<span class="hljs-string">&#x27;--context-length&#x27;</span>, <span class="hljs-built_in">type</span>=<span class="hljs-built_in">int</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L865">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="865">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                       <span class="hljs-built_in">help</span>=<span class="hljs-string">&#x27;Context length for the model (default: 512), if not provided, it will be detected from the model directory name ctxNUMBER&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L866">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="866">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    parser.add_argument(<span class="hljs-string">&#x27;--batch-size&#x27;</span>, <span class="hljs-built_in">type</span>=<span class="hljs-built_in">int</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L867">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="867">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                       <span class="hljs-built_in">help</span>=<span class="hljs-string">&#x27;Batch size for prefill (default: 64)&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L868">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="868">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    parser.add_argument(<span class="hljs-string">&#x27;--num-logits&#x27;</span>, <span class="hljs-built_in">type</span>=<span class="hljs-built_in">int</span>, default=<span class="hljs-number">8</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L869">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="869">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                       <span class="hljs-built_in">help</span>=<span class="hljs-string">&#x27;Number of logits outputs from LM head (default: 8, legacy)&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L870">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="870">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    parser.add_argument(<span class="hljs-string">&#x27;--split-lm-head&#x27;</span>, <span class="hljs-built_in">type</span>=<span class="hljs-built_in">int</span>, <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L871">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="871">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                       <span class="hljs-built_in">help</span>=<span class="hljs-string">&#x27;Number of logits splits from LM head (default: 8 for llama, 16 for qwen)&#x27;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L872">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="872">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L873">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="873">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    args = parser.parse_args()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L874">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="874">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L875">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="875">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># If meta.yaml is provided, load parameters from it</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L876">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="876">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">if</span> args.meta:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L877">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="877">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">try</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L878">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="878">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">with</span> <span class="hljs-built_in">open</span>(args.meta, <span class="hljs-string">&#x27;r&#x27;</span>) <span class="hljs-keyword">as</span> f:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L879">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="879">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                meta = yaml.safe_load(f)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L880">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="880">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            params = meta[<span class="hljs-string">&#x27;model_info&#x27;</span>][<span class="hljs-string">&#x27;parameters&#x27;</span>]<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L881">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="881">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L882">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="882">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-comment"># Set model directory to meta.yaml directory if not specified</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L883">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="883">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.d <span class="hljs-keyword">or</span> args.d == <span class="hljs-string">&#x27;.&#x27;</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L884">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="884">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                args.d = <span class="hljs-built_in">str</span>(Path(args.meta).parent)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L885">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="885">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L886">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="886">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-comment"># Build model paths based on parameters</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L887">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="887">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            prefix = params.get(<span class="hljs-string">&#x27;model_prefix&#x27;</span>, <span class="hljs-string">&#x27;llama&#x27;</span>)  <span class="hljs-comment"># Default to &#x27;llama&#x27; if not specified</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L888">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="888">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            lut_ffn = <span class="hljs-string">f&quot;_lut<span class="hljs-subst">{params[<span class="hljs-string">&#x27;lut_ffn&#x27;</span>]}</span>&quot;</span> <span class="hljs-keyword">if</span> params[<span class="hljs-string">&#x27;lut_ffn&#x27;</span>] != <span class="hljs-string">&#x27;none&#x27;</span> <span class="hljs-keyword">else</span> <span class="hljs-string">&#x27;&#x27;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L889">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="889">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            lut_lmhead = <span class="hljs-string">f&quot;_lut<span class="hljs-subst">{params[<span class="hljs-string">&#x27;lut_lmhead&#x27;</span>]}</span>&quot;</span> <span class="hljs-keyword">if</span> params[<span class="hljs-string">&#x27;lut_lmhead&#x27;</span>] != <span class="hljs-string">&#x27;none&#x27;</span> <span class="hljs-keyword">else</span> <span class="hljs-string">&#x27;&#x27;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L890">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="890">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            lut_embeddings = <span class="hljs-string">f&quot;_lut<span class="hljs-subst">{params[<span class="hljs-string">&#x27;lut_embeddings&#x27;</span>]}</span>&quot;</span> <span class="hljs-keyword">if</span> params[<span class="hljs-string">&#x27;lut_embeddings&#x27;</span>] != <span class="hljs-string">&#x27;none&#x27;</span> <span class="hljs-keyword">else</span> <span class="hljs-string">&#x27;&#x27;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L891">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="891">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            num_chunks = <span class="hljs-built_in">int</span>(params[<span class="hljs-string">&#x27;num_chunks&#x27;</span>])<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L892">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="892">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L893">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="893">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-comment"># Set model paths if not specified</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L894">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="894">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.lmhead:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L895">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="895">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                args.lmhead = <span class="hljs-string">f&#x27;<span class="hljs-subst">{prefix}</span>_lm_head<span class="hljs-subst">{lut_lmhead}</span>&#x27;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L896">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="896">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.embed:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L897">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="897">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                args.embed = <span class="hljs-string">f&#x27;<span class="hljs-subst">{prefix}</span>_embeddings<span class="hljs-subst">{lut_embeddings}</span>&#x27;</span>  <span class="hljs-comment"># Changed from lm_head to embeddings</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L898">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="898">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.ffn:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L899">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="899">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                args.ffn = <span class="hljs-string">f&#x27;<span class="hljs-subst">{prefix}</span>_FFN_PF<span class="hljs-subst">{lut_ffn}</span>_chunk_01of<span class="hljs-subst">{num_chunks:02d}</span>&#x27;</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L900">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="900">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.tokenizer:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L901">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="901">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-comment"># Check if there&#x27;s a tokenizer_path parameter in meta.yaml</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L902">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="902">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">if</span> <span class="hljs-string">&#x27;tokenizer_path&#x27;</span> <span class="hljs-keyword">in</span> params:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L903">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="903">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    args.tokenizer = params[<span class="hljs-string">&#x27;tokenizer_path&#x27;</span>]<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L904">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="904">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L905">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="905">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    <span class="hljs-comment"># Default to the model directory, but this might need manual override</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L906">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="906">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    args.tokenizer = args.d<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L907">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="907">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L908">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="908">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-comment"># Set other parameters if not overridden by command line</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L909">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="909">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> args.context_length <span class="hljs-keyword">is</span> <span class="hljs-literal">None</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L910">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="910">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                args.context_length = <span class="hljs-built_in">int</span>(params[<span class="hljs-string">&#x27;context_length&#x27;</span>])<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L911">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="911">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> args.batch_size <span class="hljs-keyword">is</span> <span class="hljs-literal">None</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L912">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="912">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                args.batch_size = <span class="hljs-built_in">int</span>(params[<span class="hljs-string">&#x27;batch_size&#x27;</span>])<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L913">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="913">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            args.num_chunks = num_chunks<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L914">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="914">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-comment"># Add num_logits parameter with default of 8, override command line if present in meta</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L915">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="915">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-string">&#x27;num_logits&#x27;</span> <span class="hljs-keyword">in</span> params:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L916">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="916">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                args.num_logits = <span class="hljs-built_in">int</span>(params[<span class="hljs-string">&#x27;num_logits&#x27;</span>])<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L917">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="917">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L918">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="918">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-comment"># Add split_lm_head parameter with default of 8</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L919">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="919">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-string">&#x27;split_lm_head&#x27;</span> <span class="hljs-keyword">in</span> params:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L920">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="920">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                args.split_lm_head = <span class="hljs-built_in">int</span>(params[<span class="hljs-string">&#x27;split_lm_head&#x27;</span>])<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L921">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="921">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L922">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="922">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                args.split_lm_head = <span class="hljs-number">8</span>  <span class="hljs-comment"># Default value for backward compatibility</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L923">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="923">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L924">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="924">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L925">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="925">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nLoaded parameters from <span class="hljs-subst">{args.meta}</span>:&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L926">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="926">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  Context Length: <span class="hljs-subst">{args.context_length}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L927">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="927">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  Batch Size: <span class="hljs-subst">{args.batch_size}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L928">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="928">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  Num Chunks: <span class="hljs-subst">{args.num_chunks}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L929">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="929">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  Num Logits: <span class="hljs-subst">{args.num_logits}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L930">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="930">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  Split LM Head: <span class="hljs-subst">{args.split_lm_head}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L931">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="931">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  Models Directory: <span class="hljs-subst">{args.d}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L932">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="932">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  Embeddings: <span class="hljs-subst">{args.embed}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L933">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="933">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  LM Head: <span class="hljs-subst">{args.lmhead}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L934">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="934">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;  FFN: <span class="hljs-subst">{args.ffn}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L935">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="935">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L936">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="936">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">except</span> Exception <span class="hljs-keyword">as</span> e:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L937">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="937">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nError loading meta.yaml: <span class="hljs-subst">{<span class="hljs-built_in">str</span>(e)}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L938">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="938">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            sys.exit(<span class="hljs-number">1</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L939">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="939">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">else</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L940">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="940">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># If no meta.yaml, set default split_lm_head if not provided</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L941">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="941">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> <span class="hljs-built_in">hasattr</span>(args, <span class="hljs-string">&#x27;split_lm_head&#x27;</span>) <span class="hljs-keyword">or</span> args.split_lm_head <span class="hljs-keyword">is</span> <span class="hljs-literal">None</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L942">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="942">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            args.split_lm_head = args.num_logits  <span class="hljs-comment"># Use num_logits as fallback</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L943">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="943">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L944">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="944">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">return</span> args<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L945">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="945">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L946">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="946">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">def</span> <span class="hljs-title function_">main</span>():<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L947">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="947">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    args = parse_args()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L948">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="948">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L949">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="949">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-comment"># Convert directory to absolute path</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L950">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="950">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    model_dir = Path(args.d).resolve()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L951">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="951">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> model_dir.exists():<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L952">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="952">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L953">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="953">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nError: Model directory not found: <span class="hljs-subst">{model_dir}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L954">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="954">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">return</span> <span class="hljs-number">1</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L955">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="955">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L956">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="956">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L957">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="957">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nUsing model directory: <span class="hljs-subst">{model_dir}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L958">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="958">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;Context length: <span class="hljs-subst">{args.context_length}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L959">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="959">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L960">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="960">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">try</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L961">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="961">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Update paths to be relative to model directory</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L962">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="962">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        args.embed = <span class="hljs-built_in">str</span>(model_dir / args.embed)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L963">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="963">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        args.ffn = <span class="hljs-built_in">str</span>(model_dir / args.ffn)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L964">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="964">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        args.lmhead = <span class="hljs-built_in">str</span>(model_dir / args.lmhead)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L965">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="965">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L966">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="966">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Handle tokenizer path separately since it&#x27;s not relative to model_dir</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L967">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="967">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> args.tokenizer <span class="hljs-keyword">is</span> <span class="hljs-literal">None</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L968">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="968">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            args.tokenizer = <span class="hljs-built_in">str</span>(model_dir)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L969">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="969">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L970">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="970">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Check if tokenizer directory exists and has required files</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L971">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="971">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        tokenizer_path = Path(args.tokenizer)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L972">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="972">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> tokenizer_path.exists():<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L973">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="973">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L974">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="974">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nError: Tokenizer directory not found: <span class="hljs-subst">{args.tokenizer}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L975">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="975">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">return</span> <span class="hljs-number">1</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L976">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="976">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L977">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="977">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Check if tokenizer has the required files</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L978">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="978">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        required_files = [<span class="hljs-string">&#x27;tokenizer.json&#x27;</span>, <span class="hljs-string">&#x27;tokenizer_config.json&#x27;</span>]<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L979">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="979">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        missing_files = [f <span class="hljs-keyword">for</span> f <span class="hljs-keyword">in</span> required_files <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> (tokenizer_path / f).exists()]<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L980">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="980">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L981">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="981">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> missing_files <span class="hljs-keyword">and</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L982">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="982">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nWarning: Tokenizer directory missing required files: <span class="hljs-subst">{missing_files}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L983">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="983">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;Current tokenizer path: <span class="hljs-subst">{args.tokenizer}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L984">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="984">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nFor Qwen models, you may need to specify the original model directory:&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L985">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="985">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;  python chat.py --meta /tmp/qwen/meta.yaml --tokenizer ~/.cache/huggingface/hub/models--Qwen--Qwen3-0.6B/snapshots/YOUR_SNAPSHOT_ID&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L986">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="986">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">&quot;\nOr add &#x27;tokenizer_path&#x27; to your meta.yaml file.&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L987">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="987">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L988">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="988">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        args.tokenizer = <span class="hljs-built_in">str</span>(Path(args.tokenizer).resolve())  <span class="hljs-comment"># Convert to absolute path</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L989">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="989">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L990">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="990">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;Using tokenizer path: <span class="hljs-subst">{args.tokenizer}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L991">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="991">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L992">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="992">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        metadata = {}<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L993">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="993">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Load models and extract metadata</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L994">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="994">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        embed_model, ffn_models, lmhead_model, metadata = load_models(args,metadata)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L995">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="995">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L996">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="996">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L997">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="997">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nMetadata befor args.context_length: <span class="hljs-subst">{metadata}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L998">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="998">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L999">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="999">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Override context length from command line if provided</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1000">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1000">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> args.context_length <span class="hljs-keyword">is</span> <span class="hljs-keyword">not</span> <span class="hljs-literal">None</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1001">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1001">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            metadata[<span class="hljs-string">&#x27;context_length&#x27;</span>] = args.context_length<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1002">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1002">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            metadata[<span class="hljs-string">&#x27;state_length&#x27;</span>] = args.context_length  <span class="hljs-comment"># Also update state_length</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1003">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1003">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1004">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1004">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nOverriding context length from command line: <span class="hljs-subst">{args.context_length}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1005">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1005">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1006">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1006">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Add num_logits to metadata (legacy support)</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1007">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1007">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        metadata[<span class="hljs-string">&#x27;num_logits&#x27;</span>] = <span class="hljs-built_in">getattr</span>(args, <span class="hljs-string">&#x27;num_logits&#x27;</span>, <span class="hljs-number">8</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1008">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1008">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1009">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1009">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Add split_lm_head to metadata (preferred)</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1010">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1010">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        metadata[<span class="hljs-string">&#x27;split_lm_head&#x27;</span>] = <span class="hljs-built_in">getattr</span>(args, <span class="hljs-string">&#x27;split_lm_head&#x27;</span>, <span class="hljs-built_in">getattr</span>(args, <span class="hljs-string">&#x27;num_logits&#x27;</span>, <span class="hljs-number">8</span>))<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1011">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1011">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1012">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1012">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1013">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1013">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nMetadata after load_models: <span class="hljs-subst">{metadata}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1014">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1014">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;Using split_lm_head value: <span class="hljs-subst">{metadata.get(<span class="hljs-string">&#x27;split_lm_head&#x27;</span>, <span class="hljs-number">8</span>)}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1015">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1015">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1016">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1016">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Load tokenizer with resolved path</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1017">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1017">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        tokenizer = initialize_tokenizer(args.tokenizer, args.<span class="hljs-built_in">eval</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1018">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1018">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> tokenizer <span class="hljs-keyword">is</span> <span class="hljs-literal">None</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1019">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1019">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">raise</span> RuntimeError(<span class="hljs-string">&quot;Failed to initialize tokenizer&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1020">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1020">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1021">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1021">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Create unified state once</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1022">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1022">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        state = create_unified_state(ffn_models, metadata[<span class="hljs-string">&#x27;context_length&#x27;</span>], args.<span class="hljs-built_in">eval</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1023">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1023">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1024">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1024">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Initialize causal mask once</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1025">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1025">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        causal_mask = initialize_causal_mask(metadata[<span class="hljs-string">&#x27;context_length&#x27;</span>], args.<span class="hljs-built_in">eval</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1026">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1026">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1027">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1027">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Warmup runs to prevent Python GIL issues with CoreML !</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1028">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1028">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.nw <span class="hljs-keyword">and</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1029">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1029">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">for</span> _ <span class="hljs-keyword">in</span> <span class="hljs-built_in">range</span>(<span class="hljs-number">2</span>):<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1030">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1030">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                chat_loop(<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1031">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1031">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    embed_model=embed_model,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1032">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1032">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    ffn_models=ffn_models,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1033">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1033">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    lmhead_model=lmhead_model,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1034">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1034">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    tokenizer=tokenizer,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1035">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1035">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    metadata=metadata,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1036">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1036">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    state=state,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1037">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1037">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    causal_mask=causal_mask,  <span class="hljs-comment"># Pass the causal mask</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1038">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1038">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    warmup=<span class="hljs-literal">True</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1039">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1039">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    auto_prompt=<span class="hljs-string">&quot;who are you?&quot;</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1040">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1040">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    no_template=args.no_template,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1041">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1041">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                    eval_mode=args.<span class="hljs-built_in">eval</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1042">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1042">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->                )<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1043">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1043">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1044">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1044">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-comment"># Main run</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1045">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1045">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        chat_loop(<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1046">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1046">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            embed_model=embed_model,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1047">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1047">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            ffn_models=ffn_models,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1048">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1048">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            lmhead_model=lmhead_model,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1049">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1049">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            tokenizer=tokenizer,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1050">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1050">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            metadata=metadata,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1051">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1051">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            state=state,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1052">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1052">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            causal_mask=causal_mask,  <span class="hljs-comment"># Pass the causal mask</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1053">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1053">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            warmup=<span class="hljs-literal">False</span>,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1054">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1054">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            auto_prompt=args.prompt,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1055">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1055">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            save_file=args.save,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1056">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1056">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            max_tokens=args.max_tokens,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1057">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1057">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            no_template=args.no_template,<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1058">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1058">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            eval_mode=args.<span class="hljs-built_in">eval</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1059">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1059">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        )<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1060">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1060">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1061">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1061">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">except</span> Exception <span class="hljs-keyword">as</span> e:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1062">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1062">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">if</span> <span class="hljs-keyword">not</span> args.<span class="hljs-built_in">eval</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1063">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1063">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-built_in">print</span>(<span class="hljs-string">f&quot;\nError: <span class="hljs-subst">{<span class="hljs-built_in">str</span>(e)}</span>&quot;</span>)<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1064">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1064">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            <span class="hljs-keyword">import</span> traceback<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1065">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1065">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->            traceback.print_exc()<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1066">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1066">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->        <span class="hljs-keyword">return</span> <span class="hljs-number">1</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1067">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1067">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1068">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1068">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    <span class="hljs-keyword">return</span> <span class="hljs-number">0</span><!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1069">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1069">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1070">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1070">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START --><span class="hljs-keyword">if</span> __name__ == <span class="hljs-string">&quot;__main__&quot;</span>:<!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1071">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1071">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->    exit(main()) <!-- HTML_TAG_END --></td>
					</tr>
					<tr class="" id="L1072">
						
						<td class="blob-line-num sticky left-0 w-1 cursor-pointer select-none pl-5 pr-3 text-right align-top text-gray-400/80 hover:text-black dark:text-gray-500 dark:hover:text-white bg-white dark:bg-gray-950" data-line-num="1072">
							<div class="absolute inset-y-0 right-0 border-r"></div></td>
						<td class="blob-line overflow-visible px-3 whitespace-pre"><!-- HTML_TAG_START -->
<!-- HTML_TAG_END --></td>
					</tr></tbody></table></div>
	</div></div></div>
				</div></section></div></main>

	</div>

		<script>
			import("\/front\/build\/kube-ab0c01c\/index.js");
			window.moonSha = "kube-ab0c01c\/";
			window.__hf_deferred = {};
		</script>

		<!-- Stripe -->
		<script>
			if (["hf.co", "huggingface.co"].includes(window.location.hostname)) {
				const script = document.createElement("script");
				script.src = "https://js.stripe.com/v3/";
				script.async = true;
				document.head.appendChild(script);
			}
		</script>
	</body>
</html>
