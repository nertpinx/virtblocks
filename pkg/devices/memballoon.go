package devices

type MemballoonModel int

const (
	MemballoonModelNone MemballoonModel = iota
	MemballoonModelVirtio
	MemballoonModelVirtioNonTransactional
	MemballoonModelVirtioTransactional
)

type Memballoon struct {
	model MemballoonModel
}

func (self MemballoonModel) String() string {
	var ret string

	switch self {
	case MemballoonModelNone:
		ret = ""
	case MemballoonModelVirtio:
		ret = "virtio-memballoon"
	case MemballoonModelVirtioNonTransactional:
		ret = "virtio-memballoon-non-transitional"
	case MemballoonModelVirtioTransactional:
		ret = "virtio-memballoon-transitional"
	}

	return ret
}

func NewMemballoon() *Memballoon {
	return &Memballoon{model: MemballoonModelNone}
}

func (self *Memballoon) SetModel(model MemballoonModel) {
	self.model = model
}

func (self Memballoon) GetModel() MemballoonModel {
	return self.model
}

func (self Memballoon) String() string {
	return self.model.String()
}