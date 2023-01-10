require 'entities'

describe CSVPlusPlus::Language::Function do
  subject { described_class.new("FOO", ["a", "b"], 3) }

  it "lowercases and converts the id to a symbol" do
    expect(subject.id).to eq :foo
  end
end

describe CSVPlusPlus::Language::FunctionCall do
  subject { described_class.new("MINUS", ["a", "b"]) }

  it "lowercases and converts the id to a symbol" do
    expect(subject.id).to eq :minus
  end
end

describe CSVPlusPlus::Language::Variable do
  subject { described_class.new("RowNUM") }

  it "lowercases and converts the id to a symbol" do
    expect(subject.id).to eq :rownum
  end
end
