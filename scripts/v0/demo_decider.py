class DeciderEncoder:
    marker_chars = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
        'u', 'v', 'w', 'x', 'y', 'z',
    ]
    
    def __init__(self, alphabet: str, encoders: list):
        if len(encoders) > len(self.marker_chars):
            raise ValueError("Too many encoders")
        self.encoders = encoders
        self.encoder_to_marker_char = {
            encoder: marker_char
            for marker_char, encoder in zip(self.marker_chars[:len(encoders)], encoders)
        }
        self.marker_char_to_encoder = {
            marker_char: encoder
            for encoder, marker_char in self.encoder_to_marker_char.items()
        }

    def encode(self, text: str) -> str:
        encoded = []
        for encoder in self.encoders:
            encoded_text = encoder.encode(text)
            encoded.append({
                'marker_char': self.encoder_to_marker_char[encoder],
                'encoded': encoded_text,
                'size': len(encoded_text.encode('utf-8'))
            })
        encoded.sort(key=lambda x: x['size'])
        return encoded[0]['marker_char'] + encoded[0]['encoded']
    
    def decode(self, encoded: str) -> str:
        marker_char = encoded[0]
        encoded_text = encoded[1:]
        encoder = self.marker_char_to_encoder[marker_char]
        return encoder.decode(encoded_text)
