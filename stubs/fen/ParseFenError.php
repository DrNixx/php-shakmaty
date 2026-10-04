<?php
namespace shakmaty\fen;

/**
 * Base exception for FEN parsing errors. Catch this type to handle any FEN
 * parse failure, or catch a concrete subclass to distinguish the failing part.
 */
class ParseFenError extends \Exception {}
