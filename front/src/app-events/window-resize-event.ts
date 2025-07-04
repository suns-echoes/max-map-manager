import { AppState } from '^state/app-state';


window.addEventListener('resize', function () {
	AppState.windowSize.set({ width: window.innerWidth, height: window.innerHeight });
});
