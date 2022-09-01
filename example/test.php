<?php

use BarCodes\Code39;
use BarCodes\Codabar;
use BarCodes\Code128;

$code = new Code39();
$code->setForeground([255, 0, 0, 255]);
$code->generatePng("1ISTHELONELIESTNUMBER", "my_barcode.png");


$code = new Codabar();
$code->generateJpeg("1A43BD4D", "my_barcode.jpg");

$code = new Code128();
$code->setBackground([0, 0, 255, 255]);
$code->generateGif("ƁxyZÀ199!*1", "my_barcode.gif");
