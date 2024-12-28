/*
 * format the time and return a string with a number followed by a unit of time 
 * @param {number} time - time in seconds 
 * @returns {string} number and unit of time
 * Example : 14h 30mins*/
export default function formatTime(time: number) {
		let hours = Math.round(time / 3600);
		let minutes = Math.round((time % 3600) / 60)
		console.log(time - hours * 60)
		let seconds = time % 60

		return hours != 0 ? `${hours}h ${minutes}mins` : `${minutes}minutes ${seconds}s`
}
