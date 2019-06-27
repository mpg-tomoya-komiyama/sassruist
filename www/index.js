import * as sassruist from '../pkg/sassruist'

const $src = document.getElementById('src')
const $result = document.getElementById('result')

function convert () {
	const text = $src.value
	const result = sassruist.return_string(text)
	$result.value = result
	$result.scrollTop = $src.scrollTop
}

document.getElementById('convert').addEventListener('click', convert)

let wait = false
$src.addEventListener('input', () => {
	if (wait) return

	wait = true
	setTimeout(() => {
		convert()
		wait = false
	}, 100)
})

const text = [
	'a {',
	'  &_b {',
	'    &_c {',
	'      color: red;',
	'    }',
	'  }', '}',
].join('\n')
$src.value = text
convert()
