# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Modifier::ValidatedModifier do
  let(:modifier) { described_class.new }

  describe '#border=' do
    subject { modifier.borders }

    { top: 'top', all: 'ALL', bottom: 'bottoM', right: 'right', left: 'LEFT' }.each do |expected_value, value|
      context value do
        before { modifier.border = value }

        it { is_expected.to(include(expected_value)) }
      end
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier.border = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#bordercolor=' do
    subject { modifier.bordercolor }

    context '#FF00AA' do
      before { modifier.bordercolor = '#FF00AA' }

      it { is_expected.to(be_a(::CSVPlusPlus::Color)) }
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier.bordercolor = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#borderstyle=' do
    subject { modifier.borderstyle }

    {
      dashed: 'DASHED',
      dotted: 'dotted',
      double: 'Double',
      solid: 'SOLId',
      solid_medium: 'SOLID_MEDIUM',
      solid_thick: 'solid_thick'
    }.each do |expected_value, value|
      context value do
        before { modifier.borderstyle = value }

        it { is_expected.to(eq(expected_value)) }
      end
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier.borderstyle = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#color=' do
    subject { modifier.color }

    context '#FF00AA' do
      before { modifier.color = '#FF00AA' }

      it { is_expected.to(be_a(::CSVPlusPlus::Color)) }
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier.color = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#expand=' do
    subject { modifier.expand }

    context '5' do
      before { modifier.expand = '5' }

      it { is_expected.to(be_a(::CSVPlusPlus::Expand)) }
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier.expand = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#fontcolor=' do
    subject { modifier.fontcolor }

    context '#F0A' do
      before { modifier.fontcolor = '#F0A' }

      it { is_expected.to(be_a(::CSVPlusPlus::Color)) }
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier.fontcolor = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#fontfamily=' do
    subject { modifier.fontfamily }

    context 'Helvetica Sans' do
      before { modifier.fontfamily = 'Helvetica Sans' }

      it { is_expected.to(eq('Helvetica Sans')) }
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier.fontfamily = '>>> Invalid$$|,,' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end

    context 'when quoted' do
      before { modifier.fontfamily = "'Helvetica Sans'" }

      it { is_expected.to(eq('Helvetica Sans')) }
    end
  end

  describe '#fontsize=' do
    subject { modifier.fontsize }

    context '22' do
      before { modifier.fontsize = '22' }

      it { is_expected.to(eq(22)) }
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier.fontsize = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#format=' do
    subject { modifier.formats }

    {
      bold: 'bold',
      italic: 'Italic',
      strikethrough: 'STRIKETHROUGH',
      underline: 'Underline'
    }.each do |expected_value, value|
      context value do
        before { modifier.format = value }

        it { is_expected.to(include(expected_value)) }
      end
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier.format = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#halign=' do
    subject { modifier.halign }

    { left: 'left', center: 'CENTER', right: 'Right' }.each do |expected_value, value|
      context value do
        before { modifier.halign = value }

        it { is_expected.to(eq(expected_value)) }
      end
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier.halign = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#note=' do
    subject { modifier.note }

    context "'this is a note'" do
      before { modifier.note = 'this is a note' }

      it { is_expected.to(eq('this is a note')) }
    end
  end

  describe '#numberformat=' do
    subject { modifier.numberformat }

    {
      currency: 'currency',
      date: 'Date',
      date_time: 'DATE_TIME',
      number: 'Number',
      percent: 'PERCENT',
      text: 'text',
      time: 'time',
      scientific: 'scientific'
    }.each do |expected_value, value|
      context value do
        before { modifier.numberformat = value }

        it { is_expected.to(eq(expected_value)) }
      end
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier.numberformat = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#valign=' do
    subject { modifier.valign }

    { top: 'top', center: 'Center', bottom: 'BOTTOM' }.each do |expected_value, value|
      context value do
        before { modifier.valign = value }

        it { is_expected.to(eq(expected_value)) }
      end
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier.valign = 'foo' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end

  describe '#validation=' do
    subject { modifier.validation }

    context 'validation that takes no args' do
      before { modifier.validation = 'blank' }

      it { is_expected.to(be_a(::CSVPlusPlus::Modifier::DataValidation)) }
    end

    context 'validation that takes two args' do
      before { modifier.validation = 'number_eq: 42' }

      it { is_expected.to(be_a(::CSVPlusPlus::Modifier::DataValidation)) }
    end

    context 'validation that takes any number of args' do
      before { modifier.validation = 'one_of_list: 1 2 3' }

      it { is_expected.to(be_a(::CSVPlusPlus::Modifier::DataValidation)) }
    end
  end

  describe '#var=' do
    subject { modifier.var }

    context 'variable_name' do
      before { modifier.var = 'variable_name' }

      it { is_expected.to(eq(:variable_name)) }
    end

    context 'with an invalid value' do
      it 'raises an error' do
        expect { modifier.var = '@)V@)Xjk ask' }
          .to(raise_error(::CSVPlusPlus::Error::ModifierValidationError))
      end
    end
  end
end
