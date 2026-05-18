from tokenizers import Tokenizer
from tokenizers.models import BPE
from tokenizers.trainers import BpeTrainer

# Instancia o BPE padrão
tokenizer = Tokenizer(BPE(unk_token="[UNK]"))
trainer = BpeTrainer(special_tokens=["[UNK]"], vocab_size=1000)

# Treina no mesmo arquivo .txt que você usou no seu código
tokenizer.train(["dom_casmurro.txt"], trainer)

# Exporta o vocabulário gerado para comparar com o seu
tokenizer.model.save(".", "bpe_oficial")
