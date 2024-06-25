export async function postFetch (
	path: string,
	options: string
) {
	return await fetch("http://localhost:8000/api" + path, {
		"method": "POST",
		"mode": "cors",
		"headers": {
			"Content-Type": "application/json"
		},
		"body" : options
	})
}