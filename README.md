# ft_linear_regression

This project is an introduction to the basic concept behind machine learning.
It contains those programs:
- `predict`: predict `y` from `x` using a linear function
- `train`: train a linear function using a gradient descent algorithm

<p align="center">
	<table>
		<tr>
			<td>
				<img src="https://github.com/Alexdelia/42-ft_linear_regression/blob/main/graph/42_provided/training.gif" alt="training on 42 provided dataset">
			</td>
			<td>
				<img src="https://github.com/Alexdelia/42-ft_linear_regression/blob/main/graph/basic/training.gif" alt="training on basic dataset">
			</td>
		</tr>
		<tr>
			<td>
				<img src="https://github.com/Alexdelia/42-ft_linear_regression/blob/main/graph/42_provided/result.png" alt="linear regression on 42 provided dataset">
			</td>
			<td>
				<img src="https://github.com/Alexdelia/42-ft_linear_regression/blob/main/graph/basic/result.png" alt="linear regression on basic dataset">
			</td>
		</tr>
	</table>
</p>

<p align="center">
	<img src="https://github.com/Alexdelia/42-ft_linear_regression/blob/main/graph/42_provided/table.png" alt="42 provided dataset post training as table">
</p>

## requirements
- [rust](https://www.rust-lang.org/tools/install)
- [make](https://www.gnu.org/software/make/#download) _(optional)_

## usage

#### via `make`:
1. compile
	```sh
	make
	```

2. `train`
	```sh
	./train
	```

3. `predict`
	```sh
	./predict 42 # predict y for x = 42
	```

#### via `cargo`:
1. `train`
	```sh
	cargo run --release --bin train
	```

2. `predict`
	```sh
	cargo run --release --bin predict 42 # predict y for x = 42
	```

