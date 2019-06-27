import * as sassruist from '../pkg/sassruist'

document.getElementById('convert').addEventListener('click', convert)

function convert () {
	const text = document.getElementById('src').value
	const result = sassruist.return_string(text)
	document.getElementById('result').value = result
}

const text = [
	'a {',
	'  &_b {',
	'    &_c {',
	'      color: red;',
	'    }',
	'  }',
	'}',
].join('\n')
document.getElementById('src').value = text
convert()

